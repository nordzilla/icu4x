// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! ## `litemap`
//!
//! `litemap` is a crate providing [`VecMap`], a highly simplistic "flat" key-value map
//! based off of a single sorted vector.
//!
//! The goal of this crate is to provide a map that is good enough for small
//! sizes, and does not carry the binary size impact of [`std::collections::HashMap`]
//! or [`std::collections::BTreeMap`].
//!
//! If binary size is not a concern, [`std::collections::BTreeMap`] may be a better choice
//! for your use case. It behaves very similarly to [`VecMap`] for less than 12 elements,
//! and upgrades itself gracefully for larger inputs.
//!

mod map;

pub use map::VecMap;
