# Architecture and strategy

The plugin gives OxiGuitar a list of devices and an audio stream. It is built on the `cpal` audio library, through a small layer that the other output plugins share, and it uses the Core Audio host of that library.

The strategy is one output plugin for each system's own sound API, so each can be installed, updated and left out on its own. OxiGuitar picks among the installed outputs.
