# Features and their tests

Everything this plugin does, and the tests that check it. The test names are the ones `cargo test` prints; a name after `macos:` is a test that only runs on Core Audio's system. `scripts/test-all.py` fails when a feature here names no test, or names a test that doesn't exist, so a new feature gets its row and its tests in the same pull request (the [Testing](testing.md) page says how to run them).

The plugin is a thin layer: opening, feeding and closing the output stream is OxiGuitar's own `oxiguitar-audio-cpal` code (through [cpal](https://github.com/RustAudio/cpal)), which this plugin uses as released. Hearing the sound needs a sound card, so the tests check everything up to it.

| Feature | What it does | Tests |
| --- | --- | --- |
| The Core Audio output | Offers the `audio-device` service `coreaudio`, and nothing else, so the player can send its sound to Core Audio. | `tests::it_offers_only_the_coreaudio_audio_device`, `macos:contract::the_plugin_file_keeps_the_contract` |
| Says when it can't play | Probing tells the player whether there's an output, and why not when there isn't (shown in the footer and Tools > Plugins). | `macos:tests::probing_says_whether_there_is_an_output` |
| Picks the output device | Lists the system's output devices by name for the footer's output chooser. | `macos:tests::listing_the_outputs_answers_with_or_without_a_device` |
| Refuses wrong requests | A damaged request to open an output, or a method it doesn't have, gets an error, never a crash. | `macos:tests::wrong_requests_are_refused` |
| Only on Core Audio's system | Its manifest limits it to macos; anywhere else it is never available and says why. | `oxiguitar_plugin_checks::the_manifest_matches_the_plugin`, `linux,windows:tests::elsewhere_it_says_it_runs_only_on_macos` |
| A working plugin file | The `.oxp` file keeps the plugin contract, with this repository's SDK version and with the newest release of OxiGuitar. | `macos:contract::the_plugin_file_keeps_the_contract`, `stage:contract (latest core)` |
