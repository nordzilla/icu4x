// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{convert::TryFrom, ptr};

use icu_datetime::{mock::zoned_datetime::MockZonedDateTime, ZonedDateTimeFormat};

use crate::{
    custom_writeable::ICU4XWriteable, locale::ICU4XLocale, provider::ICU4XDataProvider,
    zoned_datetime::ICU4XZonedDateTime,
};

use super::{components::ICU4XComponentsBag, length::ICU4XLengthBag};

/// Opaque type for use behind a pointer, is [`ZonedDateTimeFormat`]
///
/// Can be obtained via [`icu4x_zoned_datetime_format_create()`] and destroyed via [`icu4x_zoned_datetime_format_destroy()`]
pub type ICU4XZonedDateTimeFormat<'d> = ZonedDateTimeFormat<'d, 'static>;

#[repr(C)]
/// This is the result returned by [`icu4x_zoned_datetime_format_create()`]
pub struct ICU4XCreateZonedDateTimeFormatResult<'d> {
    /// Will be null if `success` is [`false`]
    pub zdtf: *mut ICU4XZonedDateTimeFormat<'d>,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`ZonedDateTimeFormat::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `zdtf` in the result if `success` is [`true`].
pub extern "C" fn icu4x_zoned_datetime_format_create_from_length_bag<'d>(
    locale: &ICU4XLocale,
    provider: &'d ICU4XDataProvider,
    options: &ICU4XLengthBag,
) -> ICU4XCreateZonedDateTimeFormatResult<'d> {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match ZonedDateTimeFormat::try_new(langid, provider, provider, &options.into()) {
        Ok(zdtf) => {
            let zdtf = Box::new(zdtf);
            ICU4XCreateZonedDateTimeFormatResult {
                zdtf: Box::into_raw(zdtf),
                success: true,
            }
        }
        Err(_) => ICU4XCreateZonedDateTimeFormatResult {
            zdtf: ptr::null_mut(),
            success: false,
        },
    }
}

#[no_mangle]
/// FFI version of [`ZonedDateTimeFormat::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `zdtf` in the result if `success` is [`true`].
pub extern "C" fn icu4x_zoned_datetime_format_create_from_components_bag<'d>(
    locale: &ICU4XLocale,
    provider: &'d ICU4XDataProvider,
    options: &ICU4XComponentsBag,
) -> ICU4XCreateZonedDateTimeFormatResult<'d> {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match ZonedDateTimeFormat::try_new(langid, provider, provider, &options.into()) {
        Ok(zdtf) => {
            let zdtf = Box::new(zdtf);
            ICU4XCreateZonedDateTimeFormatResult {
                zdtf: Box::into_raw(zdtf),
                success: true,
            }
        }
        Err(_) => ICU4XCreateZonedDateTimeFormatResult {
            zdtf: ptr::null_mut(),
            success: false,
        },
    }
}

#[no_mangle]
/// FFI version of [`ZonedDateTimeFormat::format()`]. See its docs for more details.
///
/// Returns `false` when there were errors writing to `write`
pub extern "C" fn icu4x_zoned_datetime_format_write(
    zdtf: &ICU4XZonedDateTimeFormat<'_>,
    value: &ICU4XZonedDateTime,
    write: &mut ICU4XWriteable,
) -> bool {
    use writeable::Writeable;

    let datetime = match MockZonedDateTime::try_from(value) {
        Ok(datetime) => datetime,
        Err(_) => return false,
    };

    let formatted = zdtf.format(&datetime);
    let result = formatted.write_to(write).is_ok();
    write.flush();
    result
}

#[no_mangle]
/// Destructor for [`ICU4XZonedDateTimeFormat`]
///
/// # Safety
/// `zdtf` must be a pointer to a valid [`ICU4XZonedDateTimeFormat`] constructed by
/// [`icu4x_zoned_datetime_format_create_from_length_bag()`] or
/// [`icu4x_zoned_datetime_format_create_from_components_bag()`].
pub unsafe extern "C" fn icu4x_zoned_datetime_format_destroy(pr: *mut ICU4XZonedDateTimeFormat) {
    let _ = Box::from_raw(pr);
}
