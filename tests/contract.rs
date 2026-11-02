// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The contract suite: builds this crate as an `.oxp` file and checks it
//! through OxiGuitar's own plugin host, as the app would load it. `cargo
//! test` runs it with the rest.

// The plugin is for macOS only (its manifest's conditions); elsewhere there
// is nothing to build or check.
#![cfg(target_os = "macos")]

use std::path::Path;

#[test]
fn the_plugin_file_keeps_the_contract() {
    let crate_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Builds target/oxp/audio-coreaudio.oxp: the package's name, then the file's.
    let oxp = oxiguitar_plugin_test::build(
        crate_folder,
        env!("CARGO_PKG_NAME"),
        "audio-coreaudio",
        false,
    )
    .expect("the .oxp file builds");
    let report = oxiguitar_plugin_test::check(&oxp);
    for passed in &report.passed {
        println!("ok    {passed}");
    }
    assert!(report.ok(), "failed: {:#?}", report.failed);
}
