#!/bin/sh
set -eu

if [ -z "${1:-}" ]; then
  echo "Usage: $0 <version> [npm-otp]" >&2
  exit 1
fi

version="$1"
npm_otp="${2:-}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_clean_worktree() {
  if [ -n "$(git status --porcelain)" ]; then
    echo "Working tree must be clean before publishing." >&2
    exit 1
  fi
}

stage_release_assets() {
  asset_dir="$1"
  rm -rf "$asset_dir"
  mkdir -p "$asset_dir"

  find "releases/${version}" -mindepth 2 -maxdepth 2 -type f | sort | while IFS= read -r file; do
    target="$(basename "$(dirname "$file")")"
    base="$(basename "$file")"
    cp "$file" "${asset_dir}/${target}_${base}"
  done

  cp "releases/${version}/src.tar.gz" "${asset_dir}/src.tar.gz"
  cp "releases/${version}/src.zip" "${asset_dir}/src.zip"
}

require_cmd cargo
require_cmd docker
require_cmd gh
require_cmd git
require_cmd grep
require_cmd npm
require_cmd rustup
require_cmd wasm-pack

require_clean_worktree

if [ ! -d "releases/${version}" ]; then
  echo "Expected release artifacts in releases/${version}. Run the checklist first." >&2
  exit 1
fi

current_branch="$(git branch --show-current)"
if [ -z "$current_branch" ]; then
  echo "Could not determine current branch." >&2
  exit 1
fi

if ! grep -q "^version = \"${version}\"$" Cargo.toml; then
  echo "Cargo.toml does not match version ${version}." >&2
  exit 1
fi

if ! grep -q "htmltoadf@${version}/htmltoadf.js" docs/index.html; then
  echo "docs/index.html does not reference htmltoadf@${version}." >&2
  exit 1
fi

cargo owner --list htmltoadf >/dev/null
npm whoami --registry=https://registry.npmjs.org/ >/dev/null
docker info >/dev/null
gh auth status >/dev/null

PATH="$HOME/.cargo/bin:$PATH" wasm-pack build --release --target web --out-dir pkg

if ! grep -q "\"version\": \"${version}\"" pkg/package.json; then
  echo "pkg/package.json was not generated for version ${version}." >&2
  exit 1
fi

docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --build-arg VERSION="${version}" \
  -t "wouterken/html2adf:${version}" \
  -t "wouterken/html2adf:latest" \
  --push .

if [ -n "$npm_otp" ]; then
  npm publish ./pkg --otp="$npm_otp"
else
  npm publish ./pkg
fi

cargo publish

git push origin "$current_branch"

if git rev-parse "${version}" >/dev/null 2>&1; then
  echo "Tag ${version} already exists locally." >&2
  exit 1
fi

git tag "${version}"
git push origin "${version}"

tmp_assets_dir="$(mktemp -d "/tmp/htmltoadf-release-assets-${version}.XXXXXX")"
trap 'rm -rf "$tmp_assets_dir"' EXIT INT TERM
stage_release_assets "$tmp_assets_dir"

release_notes="$(mktemp "/tmp/htmltoadf-release-notes-${version}.XXXXXX")"
trap 'rm -rf "$tmp_assets_dir"; rm -f "$release_notes"' EXIT INT TERM
cat >"$release_notes" <<EOF
## htmltoadf ${version}

### Validation
- cargo fmt --check
- cargo clippy --all-targets --all-features -- -D warnings
- cargo test
- cross-target release artifacts built via ./release.sh
- multi-arch Docker image pushed for linux/amd64 and linux/arm64
- crate published to crates.io
- WASM package published to npm
EOF

gh release create "${version}" "${tmp_assets_dir}"/* \
  --title "${version}" \
  --notes-file "$release_notes"
