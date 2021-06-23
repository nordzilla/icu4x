// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    date::{DateTimeError, GmtOffset},
    mock::time_zone::MockTimeZone,
};
use std::{convert::TryFrom, ffi::CStr, os::raw::c_char};

#[repr(C)]
pub struct ICU4XTimeZone {
    /// The GMT offset in seconds.
    pub gmt_offset: i32,
    /// The IANA time-zone identifier
    // TODO(#606) change this to BCP-47 identifier
    pub time_zone_id: *const c_char,
    /// The CLDR metazone identifier
    // TODO(#528) change this to <TBD> identifier
    pub metazone_id: *const c_char,
    /// The time variant e.g. "daylight" or "standard"
    pub time_variant: *const c_char,
}

impl TryFrom<&ICU4XTimeZone> for MockTimeZone {
    type Error = DateTimeError;
    fn try_from(other: &ICU4XTimeZone) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Self {
                gmt_offset: GmtOffset::try_new(other.gmt_offset)?,
                time_zone_id: other
                    .time_zone_id
                    .as_ref()
                    .map(|s| CStr::from_ptr(s).to_string_lossy().into()),
                metazone_id: other
                    .metazone_id
                    .as_ref()
                    .map(|s| CStr::from_ptr(s).to_string_lossy().into()),
                time_variant: other
                    .time_variant
                    .as_ref()
                    .map(|s| CStr::from_ptr(s).to_string_lossy().parse().unwrap()),
            })
        }
    }
}
