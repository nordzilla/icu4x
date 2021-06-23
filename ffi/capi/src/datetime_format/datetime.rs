// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    custom_writeable::ICU4XWriteable,
    datetime::ICU4XDateTime,
    datetime_format::{components::ICU4XComponentsBag, length::ICU4XLengthBag},
    locale::ICU4XLocale,
    provider::ICU4XDataProvider,
};
use icu_datetime::{mock::datetime::MockDateTime, DateTimeFormat, DateTimeFormatOptions};
use std::{convert::TryFrom, ptr};

/// Opaque type for use behind a pointer, is [`DateTimeFormat`]
///
/// Can be obtained via [`icu4x_datetime_format_create()`] and destroyed via [`icu4x_datetime_format_destroy()`]
pub type ICU4XDateTimeFormat<'d> = DateTimeFormat<'d, 'static>;

#[repr(C)]
/// This is the result returned by [`icu4x_datetime_format_create()`]
pub struct ICU4XCreateDateTimeFormatResult<'d> {
    /// Will be null if `success` is [`false`]
    pub dtf: *mut ICU4XDateTimeFormat<'d>,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`DateTimeFormat::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `dtf` in the result if `success` is [`true`].
pub extern "C" fn icu4x_datetime_format_create_from_length_bag<'d>(
    locale: &ICU4XLocale,
    provider: &'d ICU4XDataProvider,
    options: &ICU4XLengthBag,
) -> ICU4XCreateDateTimeFormatResult<'d> {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match DateTimeFormat::try_new(langid, provider, &options.into()) {
        Ok(dtf) => {
            let dtf = Box::new(dtf);
            ICU4XCreateDateTimeFormatResult {
                dtf: Box::into_raw(dtf),
                success: true,
            }
        }
        Err(_) => ICU4XCreateDateTimeFormatResult {
            dtf: ptr::null_mut(),
            success: false,
        },
    }
}

#[no_mangle]
/// FFI version of [`DateTimeFormat::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `dtf` in the result if `success` is [`true`].
pub extern "C" fn icu4x_datetime_format_create_from_components_bag<'d>(
    locale: &ICU4XLocale,
    provider: &'d ICU4XDataProvider,
    options: &ICU4XComponentsBag,
) -> ICU4XCreateDateTimeFormatResult<'d> {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match DateTimeFormat::try_new(langid, provider, &options.into()) {
        Ok(dtf) => {
            let dtf = Box::new(dtf);
            ICU4XCreateDateTimeFormatResult {
                dtf: Box::into_raw(dtf),
                success: true,
            }
        }
        Err(_) => ICU4XCreateDateTimeFormatResult {
            dtf: ptr::null_mut(),
            success: false,
        },
    }
}

#[no_mangle]
/// FFI version of [`DateTimeFormat::format()`]. See its docs for more details.
///
/// Returns `false` when there were errors writing to `write`
pub extern "C" fn icu4x_datetime_format_write(
    dtf: &ICU4XDateTimeFormat<'_>,
    value: &ICU4XDateTime,
    write: &mut ICU4XWriteable,
) -> bool {
    use writeable::Writeable;

    let datetime = match MockDateTime::try_from(value) {
        Ok(datetime) => datetime,
        Err(_) => return false,
    };

    let formatted = dtf.format(&datetime);
    let result = formatted.write_to(write).is_ok();
    write.flush();
    result
}

#[no_mangle]
/// Destructor for [`ICU4XDateTimeFormat`]
///
/// # Safety
/// `dtf` must be a pointer to a valid [`ICU4XDateTimeFormat`] constructed by
/// [`icu4x_datetime_format_create_from_length_bag()`] or
/// [`icu4x_datetime_format_create_from_components_bag()`].
pub unsafe extern "C" fn icu4x_datetime_format_destroy(pr: *mut ICU4XDateTimeFormat) {
    let _ = Box::from_raw(pr);
}

impl From<&ICU4XLengthBag> for DateTimeFormatOptions {
    fn from(bag: &ICU4XLengthBag) -> Self {
        Self::Length(bag.into())
    }
}

impl From<&ICU4XComponentsBag> for DateTimeFormatOptions {
    fn from(bag: &ICU4XComponentsBag) -> Self {
        Self::Components(bag.into())
    }
}
