#!/usr/bin/env bash

###
## A build script, intended to be run manually from macOS.
##
## This script will:
## * Build the application normally, in release mode
## * Build a .deb file in a Docker container
## * Build a Linux musl release in a Docker container
###
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
PROJECT=$(cargo read-manifest | jq -rM ".name")
ARTIFACT_DIRECTORY="$DIR/../artifacts"

build_macos() {
  local version="$1"
  local tarname="$PROJECT-$version-x86_64-apple-darwin.tar.gz"

  echo "Building $version for macOS..."
  cargo build --release
  pushd "$DIR/../target/release"
  tar czvf "$tarname" "$PROJECT"
  mv "$tarname" "$ARTIFACT_DIRECTORY"
  popd
  cargo clean

  echo "macOS build: $ARTIFACT_DIRECTORY/$tarname"
}

build_debian() {
  local version="$1"
  local debname="${PROJECT}_${version}_amd64.deb"

  echo "Building $version for Debian..."
  docker build -t rust-deb-build - < "$DIR/Dockerfile-debian"
  docker run --rm -it -v "$(pwd)":/usr/src/app -w /usr/src/app rust-deb-build \
    cargo deb
  mv "target/debian/$debname" "$ARTIFACT_DIRECTORY"
  cargo clean

  echo "Debian build: $ARTIFACT_DIRECTORY/$debname"
}

build_linux() {
  local version="$1"
  local tarname="$PROJECT-$version-x86_64-unknown-linux-musl.tar.gz"

  echo "Building $version for Linux (musl)..."
  docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder \
    cargo build --release
  pushd "$DIR/../target/x86_64-unknown-linux-musl/release"
  tar czvf "$tarname" "$PROJECT"
  mv "$tarname" "$ARTIFACT_DIRECTORY"
  popd
  cargo clean

  echo "Linux (musl) build: $ARTIFACT_DIRECTORY/$tarname"
}

main() {
  local version="$1"

  if [[ -z "$version" ]]; then
    version="$(cargo read-manifest | jq -rM ".version")"

    echo "No version passed. Using version from Cargo.toml ($version)."
  fi

  mkdir -p "$ARTIFACT_DIRECTORY"
  rm "$ARTIFACT_DIRECTORY"/*
  build_macos "$version"
  build_debian "$version"
  build_linux "$version"
}

main "$@"
