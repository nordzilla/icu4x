use icu_datetime::options::preferences::{Bag, HourCycle};

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ICU4XPreferencesBag {
    pub hour_cycle: *const ICU4XHourCycle,
}

impl From<ICU4XPreferencesBag> for Bag {
    fn from(other: ICU4XPreferencesBag) -> Self {
        unsafe {
            Self {
                hour_cycle: other.hour_cycle.as_ref().cloned().map(Into::into),
            }
        }
    }
}

c_enum! {
    pub c_enum ICU4XHourCycle is HourCycle {
        H24,
        H23,
        H12,
        H11,
    }
}
