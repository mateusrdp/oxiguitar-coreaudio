#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors
#
# Builds the plugin's release file for the system this runs on:
#
#   dist/audio-coreaudio-VERSION-SYSTEM-ARCH.oxp
#
# such as dist/audio-coreaudio-1.0.0-macos-universal.oxp, named the way OxiGuitar's plugins
# are (docs/plugins/template in github.com/mateusrdp/oxiguitar). This plugin is for macOS only (its manifest says so): it builds both processors and joins them into one universal file (needs both Rust targets: rustup target add aarch64-apple-darwin x86_64-apple-darwin).
# .github/workflows/release.yml runs it.

set -euo pipefail

# The plugin's file name, and the crate's library name (the package's name
# with _ for -).
NAME=audio-coreaudio
LIBRARY=oxiguitar_audio_coreaudio_plugin

cd "$(dirname "$0")"
version="$(cargo pkgid)"
version="${version##*[#@]}"
host="$(rustc -vV | sed -n 's/^host: //p')"
mkdir -p dist

build() {
    cargo rustc --locked --release --features oxp --crate-type cdylib "$@"
}

case "$host" in
    *apple*)
        out="dist/$NAME-$version-macos-universal.oxp"
        for target in aarch64-apple-darwin x86_64-apple-darwin; do
            build --target "$target"
        done
        lipo -create -output "$out" \
            "target/aarch64-apple-darwin/release/lib$LIBRARY.dylib" \
            "target/x86_64-apple-darwin/release/lib$LIBRARY.dylib"
        ;;
    *)
        echo "build-oxp.sh: $NAME is for macOS only (this is $host)" >&2
        exit 1
        ;;
esac
echo "$out"
