// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! OxiGuitar's audio-device plugin for Core Audio (plugin interface 1.3): the
//! service `audio-device` `coreaudio`, through cpal's Core Audio host
//! (`oxiguitar-audio-cpal`). Built only for macos; elsewhere the crate is
//! a plugin whose service is never available, so the workspace builds on
//! every system.

use oxiguitar_plugin::{Availability, Error, Host, Plugin, Service};

/// The plugin.
#[derive(Debug)]
pub struct Audio {
    #[cfg(target_os = "macos")]
    device: oxiguitar_audio_cpal::CpalDevice,
}

impl Plugin for Audio {
    fn start(host: Host) -> Result<Audio, Error> {
        #[cfg(not(target_os = "macos"))]
        let _ = host;
        Ok(Audio {
            #[cfg(target_os = "macos")]
            device: oxiguitar_audio_cpal::CpalDevice::new(
                host,
                cpal::HostId::CoreAudio,
                "Core Audio",
            ),
        })
    }

    fn service(&self, interface: &str, id: &str) -> Option<&dyn Service> {
        (interface == "audio-device" && id == "coreaudio").then_some(self as &dyn Service)
    }
}

impl Service for Audio {
    fn probe(&self) -> Availability {
        #[cfg(target_os = "macos")]
        return self.device.probe();
        #[cfg(not(target_os = "macos"))]
        Availability::Unavailable("it runs only on macos".into())
    }

    fn call(&self, method: &str, input: &[u8]) -> Result<Vec<u8>, Error> {
        #[cfg(target_os = "macos")]
        return self.device.call(method, input);
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (method, input);
            Err(Error::new(
                oxiguitar_plugin::ErrorKind::Unsupported,
                "it runs only on macos",
            ))
        }
    }
}

oxiguitar_plugin::export_plugin!(Audio);

#[cfg(test)]
mod tests {
    use super::Audio;
    use oxiguitar_plugin::{Availability, ErrorKind, Host, Plugin, Service};

    fn plugin() -> Audio {
        Audio::start(Host::detached()).expect("the plugin starts")
    }

    #[test]
    fn it_offers_only_the_coreaudio_audio_device() {
        let p = plugin();
        assert!(p.service("audio-device", "coreaudio").is_some());
        assert!(p.service("audio-device", "something-else").is_none());
        assert!(p.service("format", "coreaudio").is_none());
    }

    /// With or without a sound card (CI has none), probing answers, and
    /// says why when the device can't be used.
    #[cfg(target_os = "macos")]
    #[test]
    fn probing_says_whether_there_is_an_output() {
        match plugin().probe() {
            Availability::Available => {}
            Availability::Unavailable(why) => assert!(!why.trim().is_empty()),
        }
    }

    /// The device list comes back as names, or as an error that says why.
    #[cfg(target_os = "macos")]
    #[test]
    fn listing_the_outputs_answers_with_or_without_a_device() {
        use oxiguitar_plugin::api::audio_device::DEVICES;
        let answer: Result<Vec<u8>, _> = plugin().call(DEVICES, &[]);
        match answer {
            Ok(answer) => {
                let names: Vec<String> =
                    oxiguitar_plugin::api::decode(answer.as_slice()).expect("a list of names");
                assert!(names.iter().all(|n| !n.is_empty()));
            }
            Err(e) => assert!(!e.message.trim().is_empty()),
        }
    }

    /// A damaged request to open an output, and a method it doesn't have,
    /// are refused with an error rather than a crash.
    #[cfg(target_os = "macos")]
    #[test]
    fn wrong_requests_are_refused() {
        use oxiguitar_plugin::api::audio_device::OPEN;
        let p = plugin();
        assert!(p.call(OPEN, b"not a request").is_err());
        let e = p.call("no-such-method", &[]).unwrap_err();
        assert_eq!(e.kind, ErrorKind::NotFound);
    }

    /// Elsewhere it is never available and says it runs only on macos.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn elsewhere_it_says_it_runs_only_on_macos() {
        let p = plugin();
        assert_eq!(
            p.probe(),
            Availability::Unavailable("it runs only on macos".into())
        );
        assert_eq!(
            p.call("devices", &[]).unwrap_err().kind,
            ErrorKind::Unsupported
        );
    }
}
