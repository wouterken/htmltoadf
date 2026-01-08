#!/bin/sh
set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

version="$1"

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test

printf "%s\n" "${version}" | ./release.sh

docker buildx build --platform linux/amd64 --build-arg VERSION="${version}" \
  -t "wouterken/html2adf:${version}" --load .

echo "Release artifacts:"
ls "releases/${version}"
