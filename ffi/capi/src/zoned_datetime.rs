// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::convert::{TryFrom, TryInto};

use icu_datetime::{date::DateTimeError, mock::zoned_datetime::MockZonedDateTime};

use crate::{datetime::ICU4XDateTime, time_zone::ICU4XTimeZone};

#[repr(C)]
pub struct ICU4XZonedDateTime {
    /// The datetime component.
    pub datetime: ICU4XDateTime,
    /// The time zone component.
    pub time_zone: ICU4XTimeZone,
}

impl TryFrom<&ICU4XZonedDateTime> for MockZonedDateTime {
    type Error = DateTimeError;
    fn try_from(other: &ICU4XZonedDateTime) -> Result<Self, Self::Error> {
        Ok(Self {
            datetime: (&other.datetime).try_into()?,
            time_zone: (&other.time_zone).try_into()?,
        })
    }
}
