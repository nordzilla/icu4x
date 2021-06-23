use icu_datetime::options::length::{Bag, Date, Time};

use super::preferences::ICU4XPreferencesBag;

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ICU4XLengthBag {
    pub date: *const ICU4XDateLength,
    pub time: *const ICU4XTimeLength,
    pub preferences: *const ICU4XPreferencesBag,
}

impl From<&ICU4XLengthBag> for Bag {
    fn from(other: &ICU4XLengthBag) -> Self {
        unsafe {
            Self {
                date: other.date.as_ref().cloned().map(Into::into),
                time: other.time.as_ref().cloned().map(Into::into),
                preferences: other.preferences.as_ref().cloned().map(Into::into),
            }
        }
    }
}

c_enum! {
    pub c_enum ICU4XDateLength is Date {
        Full,
        Long,
        Medium,
        Short,
    }
}

c_enum! {
    pub c_enum ICU4XTimeLength is Time {
        Full,
        Long,
        Medium,
        Short,
    }
}
