// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_ZONED_DATETIME_FORMAT_H
#define ICU4X_ZONED_DATETIME_FORMAT_H

#include "datetime_format.h"
#include "zoned_datetime.h"
#include "provider.h"
#include "locale.h"
#include "custom_writeable.h"

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XZonedDateTimeFormat ICU4XZonedDateTimeFormat;

typedef struct {
    ICU4XZonedDateTimeFormat* zdtf;
    bool success;
} ICU4XCreateZonedDateTimeFormatResult;

ICU4XCreateZonedDateTimeFormatResult icu4x_zoned_datetime_format_create_from_length_bag(const ICU4XLocale* locale, const ICU4XDataProvider* provider, const ICU4XLengthBag* bag);
ICU4XCreateZonedDateTimeFormatResult icu4x_zoned_datetime_format_create_from_components_bag(const ICU4XLocale* locale, const ICU4XDataProvider* provider, const ICU4XComponentsBag* bag);

bool icu4x_zoned_datetime_format_write(const ICU4XZonedDateTimeFormat* dtf, const ICU4XZonedDateTime* value, ICU4XWriteable* write);

void icu4x_zoned_datetime_format_destroy(ICU4XZonedDateTimeFormat* dtf);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_ZONED_DATETIME_FORMAT_H
