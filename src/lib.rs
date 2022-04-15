//! [GitHub](https://github.com/virtual-puppet-project/vp3-ifm)
//!
//! This crate is the iFacialMocap/Facemotion3D plugin for the [Virtual Puppet Project](https://vss-project.com/).
//!
//! To use this crate, you need to import this crate to your Godot project.
//!
//! This works by listening for UDP packets from the iFacialMocap/Facemotion3D client on your iOS device, which contains a unique key-value store format for facial data.
//! The result is a dictionary of Perfect Sync blendshapes.
//! # Perfect Sync
//! Perfect Sync is a superset of the VRM format that contains various blendshapes that allow for more expressive facial expressions.
//! It was created to make use of Apple's [ARKit](https://developer.apple.com/documentation/arkit) blendshapes, which are also used for Animojis and Memojis.
//!
//! More info about Perfect Sync can be found [here](https://hinzka.hatenablog.com/entry/2020/08/15/145040) (In Japanese).
use gdnative::prelude::*;
pub mod ifm;
pub mod ifm_data;

// Add docs for ifm_data module


fn init(handle: InitHandle) {
    handle.add_class::<ifm::Ifacialmocap>();
}

godot_init!(init);