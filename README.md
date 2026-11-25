# oxiguitar-coreaudio

The Core Audio output plugin for [OxiGuitar](https://github.com/mateusrdp/oxiguitar), a free, open-source guitar tablature and score viewer, editor and player for Windows, macOS and Linux. It plays OxiGuitar's sound through Core Audio, the sound system of macOS.

In plain words: OxiGuitar works out the sound, and an output plugin hands it to the operating system's sound system on time, batch after batch. Every system has its own sound system, so each gets its own plugin. The [manual](https://mateusrdp.github.io/oxiguitar-coreaudio/) explains latency and buffers.

- Plugin file `audio-coreaudio.oxp`, id `oxiguitar.audio-coreaudio`, macOS only: one universal file for Apple Silicon and Intel.
- It shares its code with the other outputs, oxiguitar-pipewire, oxiguitar-alsa and oxiguitar-wasapi: a stream on one of cpal's hosts, in [oxiguitar-audio-cpal](https://github.com/mateusrdp/oxiguitar-audio-cpal), which this plugin depends on by tag.
- It plays sound only on your machine: it uploads and downloads nothing.
- The macOS `.pkg` and the Homebrew cask are not signed or notarised (no Apple fees): the `.pkg` installs for you alone, without a password, and a file installed that way is not quarantined.

## Manual

The manual is at <https://mateusrdp.github.io/oxiguitar-coreaudio/>. Its source is in [docs/manual](docs/manual). It covers what the plugin is for, how it works, what it needs and how to install it.

## Install

Every [release](https://github.com/mateusrdp/oxiguitar-coreaudio/releases) holds a universal zip, a `.pkg` that installs for you alone (no password), and a `SHA256SUMS` file. The [manual's install page](https://mateusrdp.github.io/oxiguitar-coreaudio/install.html) says which one to use and how to keep it updated. The plugin needs OxiGuitar 0.23 or later, which reads plugin files as archives (the plugin asks for interface 1.3).

## Build and test

Rust 1.99 or later from [rustup](https://rustup.rs). The plugin SDK (`v0.11.0`) and the shared audio layer, [oxiguitar-audio-cpal](https://github.com/mateusrdp/oxiguitar-audio-cpal) (`v0.1.0`), come from their repositories by git tag; nothing of OxiGuitar's is on crates.io. The SDK tag must be the one oxiguitar-audio-cpal itself uses, so the two tags and cpal's version move together, the audio layer first. After moving any of them, run `cargo update -p oxiguitar-audio-cpal -p cpal` and commit `Cargo.lock`. `deny.toml` allows both git sources. What a repository that depends on the audio layer must do is on its [How to use it](https://mateusrdp.github.io/oxiguitar-audio-cpal/use-cases.html) page.

```sh
cargo test          # unit tests, and the contract suite on the built .oxp file
./build-oxp.sh      # dist/audio-coreaudio-VERSION-SYSTEM-ARCH.oxp
python3 scripts/test-all.py   # every test, as the manual's Testing page says
```

## History

This plugin lived in the OxiGuitar repository until its version 0.11.0. Its history came along; an issue number such as `mateusrdp/oxiguitar#123` in an old commit message is in that repository.

## Credits

Built on [cpal](https://github.com/RustAudio/cpal) (Apache-2.0), [coreaudio-rs](https://github.com/RustAudio/coreaudio-rs) and [objc2](https://github.com/madsmtm/objc2), through OxiGuitar's shared `oxiguitar-audio-cpal` layer, and OxiGuitar's plugin SDK. Thanks to the RustAudio community. The manual's [Credits](https://mateusrdp.github.io/oxiguitar-coreaudio/credits.html) page lists every project with its authors and licence. Thank you to all of them.

## Licence

GPL-3.0-or-later. See LICENSE. Contributions are welcome under the same terms; see [CONTRIBUTING.md](CONTRIBUTING.md).
