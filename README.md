# oxiguitar-coreaudio

The Core Audio output plugin for [OxiGuitar](https://github.com/mateusrdp/oxiguitar), a free, open-source guitar tablature and score viewer, editor and player for Windows, macOS and Linux. It plays OxiGuitar's sound through Core Audio, the sound system of macOS.

- Plugin file `audio-coreaudio.oxp`, id `oxiguitar.audio-coreaudio`, macOS only: one universal file for Apple Silicon and Intel.
- It shares its code with the other outputs, [oxiguitar-pipewire](https://github.com/mateusrdp/oxiguitar-pipewire), [oxiguitar-alsa](https://github.com/mateusrdp/oxiguitar-alsa) and [oxiguitar-wasapi](https://github.com/mateusrdp/oxiguitar-wasapi): a stream on one of cpal's hosts, in `oxiguitar-audio-cpal`, which lives in the OxiGuitar repository and is tagged with the SDK.
- It plays sound only on your machine: it uploads and downloads nothing.
- The macOS `.pkg` and the Homebrew cask are not signed or notarised (no Apple fees): the `.pkg` installs for you alone, without a password, and a file installed that way is not quarantined.

## Install

Every [release](https://github.com/mateusrdp/oxiguitar-coreaudio/releases) holds a universal zip, a `.pkg` that installs for you alone (no password), and a `SHA256SUMS` file. The OxiGuitar manual's [install page](https://mateusrdp.github.io/oxiguitar/installing.html) says which one to use and how to keep the plugin updated through your package manager. The plugin needs OxiGuitar 0.11 or later (plugin interface 1.4).

## Build and test

Rust 1.99 or later from [rustup](https://rustup.rs). The plugin SDK comes from OxiGuitar's repository by git tag (`v0.11.0`); nothing of OxiGuitar's is on crates.io.

```sh
cargo test          # unit tests, and the contract suite on the built .oxp file
./build-oxp.sh      # dist/audio-coreaudio-VERSION-SYSTEM-ARCH.oxp
```

## History

This plugin lived in the [OxiGuitar repository](https://github.com/mateusrdp/oxiguitar) until its version 0.11.0. Its history came along; an issue number such as `mateusrdp/oxiguitar#123` in an old commit message is in that repository.

## Licence

GPL-3.0-or-later. See LICENSE. Contributions are welcome under the same terms; see [CONTRIBUTING.md](CONTRIBUTING.md).
