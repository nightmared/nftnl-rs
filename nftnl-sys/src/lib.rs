// Copyright 2018 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Low level FFI bindings to [`libnftnl`], a userspace library providing a low-level netlink
//! programming interface (API) to the in-kernel nf_tables subsystem.
//!
//! See [`nftnl`] for a higher level safe abstraction.
//!
//! # Linking to libmnl and libnftnl
//!
//! By default this crate uses pkg-config to find and link to its C dependencies, [`libmnl`] and
//! [`libnftnl`]. To manually configure where to look for these libraries, set the environment
//! variables `LIBMNL_LIB_DIR` and `LIBNFTNL_LIB_DIR` to point to the directories where `libmnl.so`
//! (or `libmnl.a`) and `libnftnl.so` (or `libnftnl.a`) reside.
//!
//! # Selecting version of `libnftnl`
//!
//! This crate has bindings for most versions of [`libnftnl`]. All bindings are generated by
//! [`bindgen`] via the `generate_bindings.sh` script in this repository.
//!
//! Only one version of `libnftnl` can be exposed via this crate. By default the crate exports the
//! bindings for the oldest supported version (`libnftnl-1.0.6`). To get newer versions activate the
//! corresponding features. See `Cargo.toml` for available features/versions.
//!
//! So for example, to get bindings to `libnftnl-1.0.9` depend on this crate like this:
//! ```toml
//! [dependencies]
//! nftnl-sys = { version = "0.1", features = ["nftnl-1-0-9"] }
//! ```
//!
//! [`libnftnl`]: https://netfilter.org/projects/libnftnl/
//! [`libmnl`]: https://netfilter.org/projects/libmnl/
//! [`nftnl`]: https://crates.io/crates/nftnl
//! [`bindgen`]: https://crates.io/crates/bindgen

#![no_std]
#![cfg(target_os = "linux")]
#![allow(non_camel_case_types)]

pub use libc;

cfg_if::cfg_if! {
    if #[cfg(feature = "nftnl-1-2-0")] {
        mod nftnl_1_2_0;
        pub use self::nftnl_1_2_0::*;
    } else if #[cfg(feature = "nftnl-1-1-2")] {
        mod nftnl_1_1_2;
        pub use self::nftnl_1_1_2::*;
    } else if #[cfg(feature = "nftnl-1-1-1")] {
        mod nftnl_1_1_1;
        pub use self::nftnl_1_1_1::*;
    } else if #[cfg(feature = "nftnl-1-1-0")] {
        mod nftnl_1_1_0;
        pub use self::nftnl_1_1_0::*;
    } else if #[cfg(feature = "nftnl-1-0-9")] {
        mod nftnl_1_0_9;
        pub use self::nftnl_1_0_9::*;
    } else if #[cfg(feature = "nftnl-1-0-8")] {
        mod nftnl_1_0_8;
        pub use self::nftnl_1_0_8::*;
    } else if #[cfg(feature = "nftnl-1-0-7")] {
        mod nftnl_1_0_7;
        pub use self::nftnl_1_0_7::*;
    } else {
        mod nftnl_1_0_6;
        pub use self::nftnl_1_0_6::*;
    }
}
