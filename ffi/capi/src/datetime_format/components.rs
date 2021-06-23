use icu_datetime::options::components::{Bag, Month, Numeric, Text, TimeZoneName};

use super::preferences::ICU4XPreferencesBag;

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ICU4XComponentsBag {
    pub era: *const ICU4XText,
    pub year: *const ICU4XNumeric,
    pub month: *const ICU4XMonth,
    pub day: *const ICU4XNumeric,
    pub weekday: *const ICU4XText,

    pub hour: *const ICU4XNumeric,
    pub minute: *const ICU4XNumeric,
    pub second: *const ICU4XNumeric,

    pub time_zone_name: *const ICU4XTimeZoneName,

    pub preferences: *const ICU4XPreferencesBag,
}

impl From<&ICU4XComponentsBag> for Bag {
    fn from(other: &ICU4XComponentsBag) -> Self {
        unsafe {
            Self {
                era: other.era.as_ref().cloned().map(Into::into),
                year: other.year.as_ref().cloned().map(Into::into),
                month: other.month.as_ref().cloned().map(Into::into),
                day: other.day.as_ref().cloned().map(Into::into),
                weekday: other.weekday.as_ref().cloned().map(Into::into),
                hour: other.hour.as_ref().cloned().map(Into::into),
                minute: other.minute.as_ref().cloned().map(Into::into),
                second: other.second.as_ref().cloned().map(Into::into),
                time_zone_name: other.time_zone_name.as_ref().cloned().map(Into::into),
                preferences: other.preferences.as_ref().cloned().map(Into::into),
            }
        }
    }
}

c_enum! {
    pub c_enum ICU4XNumeric is Numeric {
        Numeric,
        TwoDigit,
    }
}

c_enum! {
    pub c_enum ICU4XText is Text {
        Long,
        Short,
        Narrow,
    }
}

c_enum! {
    pub c_enum ICU4XMonth is Month {
        Numeric,
        TwoDigit,
        Long,
        Short,
        Narrow,
    }
}

c_enum! {
    pub c_enum ICU4XTimeZoneName is TimeZoneName {
        Long,
        Short,
    }
}
