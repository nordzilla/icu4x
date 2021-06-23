use std::convert::{TryFrom, TryInto};

use icu_datetime::{date::DateTimeError, mock::datetime::MockDateTime};

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(C)]
pub struct ICU4XDateTime {
    /// ISO-8601 year (proleptic Gregorian).
    pub year: i32,

    /// 0-based month index.
    pub month: u32,

    /// 0-based day index.
    pub day: u32,

    /// 0-based hour.
    pub hour: u8,

    /// 0-based minute.
    pub minute: u8,

    /// 0-based second.
    pub second: u8,
}

impl TryFrom<&ICU4XDateTime> for MockDateTime {
    type Error = DateTimeError;
    fn try_from(other: &ICU4XDateTime) -> Result<Self, Self::Error> {
        Ok(Self {
            year: other.year.into(),
            month: other.month.into(),
            day: other.day.into(),
            hour: other.hour.try_into()?,
            minute: other.minute.try_into()?,
            second: other.second.try_into()?,
        })
    }
}
