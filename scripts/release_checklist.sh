#!/bin/sh
set -eu

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

version="$1"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd rustup
require_cmd docker
require_cmd zig

if ! cargo xwin --help >/dev/null 2>&1; then
  echo "Missing required cargo subcommand: cargo xwin" >&2
  exit 1
fi

if ! cargo zigbuild --help >/dev/null 2>&1; then
  echo "Missing required cargo subcommand: cargo zigbuild" >&2
  exit 1
fi

docker info >/dev/null

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test

printf "%s\n" "${version}" | ./release.sh

docker buildx build --platform linux/amd64 --build-arg VERSION="${version}" \
  -t "wouterken/html2adf:${version}" --load .

echo "Release artifacts:"
ls "releases/${version}"

echo "Checklist complete for ${version}."
