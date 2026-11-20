# Installing

You need OxiGuitar first: [its manual](https://mateusrdp.github.io/oxiguitar/) says how to get it. This page is about the Core Audio output plugin.

## From a release

Each release on the [releases page](https://github.com/mateusrdp/oxiguitar-coreaudio/releases) has a universal zip for Apple silicon and Intel, a `.pkg` that installs for you alone (no administrator password), and a `SHA256SUMS` file to check the downloads against. Take the file that matches your system.

## Copying the file yourself

A plugin is a single file ending in `.oxp`. Unzip the release for your system and put `audio-coreaudio.oxp` in your plugins folder:

| System | Your plugins folder |
| --- | --- |
| macOS | `~/Library/Application Support/OxiGuitar/plugins/` |

Start OxiGuitar again. **Tools → Plugins…** lists what it found. The same menu switches a plugin off, and deleting the file removes it.

On macOS, a plugin file you downloaded yourself may be refused as coming from an unverified developer. Clear the download mark and start OxiGuitar again:

```sh
xattr -d com.apple.quarantine ~/Library/Application\ Support/OxiGuitar/plugins/*.oxp
```

A plugin installed with the `.pkg` is not marked this way.
