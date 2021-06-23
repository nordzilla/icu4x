// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_DATETIME_FORMAT_H
#define ICU4X_DATETIME_FORMAT_H

#include "datetime.h"
#include "provider.h"
#include "locale.h"
#include "custom_writeable.h"

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XDateTimeFormat ICU4XDateTimeFormat;

typedef struct {
    ICU4XDateTimeFormat* dtf;
    bool success;
} ICU4XCreateDateTimeFormatResult;

typedef enum {
    ICU4XHourCycle_H24,
    ICU4XHourCycle_H23,
    ICU4XHourCycle_H12,
    ICU4XHourCycle_H11,
} ICU4XHourCycle;

typedef struct {
    ICU4XHourCycle* hour_cycle;
} ICU4XPreferencesBag;

typedef enum {
    ICU4XDateLength_Full,
    ICU4XDateLength_Long,
    ICU4XDateLength_Medium,
    ICU4XDateLength_Short,
} ICU4XDateLength;

typedef enum {
    ICU4XTimeLength_Full,
    ICU4XTimeLength_Long,
    ICU4XTimeLength_Medium,
    ICU4XTimeLength_Short,
} ICU4XTimeLength;

typedef struct {
    ICU4XDateLength* date;
    ICU4XTimeLength* time;
    ICU4XPreferencesBag* preferences;
} ICU4XLengthBag;

typedef enum {
    ICU4XNumeric_Numeric,
    ICU4XNumeric_TwoDigit,
} ICU4XNumeric;

typedef enum {
    ICU4XText_Long,
    ICU4XText_Short,
    ICU4XText_Narrow,
} ICU4XText;

typedef enum {
    ICU4XMonth_Numeric,
    ICU4XMonth_TwoDigit,
    ICU4XMonth_Long,
    ICU4XMonth_Short,
    ICU4XMonth_Narrow,
} ICU4XMonth;

typedef enum {
    ICU4XTimeZoneName_Long,
    ICU4XTimeZoneName_Short,
} ICU4XTimeZoneName;

typedef struct {
    ICU4XText* era;
    ICU4XNumeric* year;
    ICU4XMonth* month;
    ICU4XNumeric* day;
    ICU4XText* weekday;

    ICU4XNumeric* hour;
    ICU4XNumeric* minute;
    ICU4XNumeric* second;

    ICU4XTimeZoneName* time_zone_name;

    ICU4XPreferencesBag* preferences;
} ICU4XComponentsBag;

ICU4XCreateDateTimeFormatResult icu4x_datetime_format_create_from_length_bag(const ICU4XLocale* locale, const ICU4XDataProvider* provider, const ICU4XLengthBag* bag);
ICU4XCreateDateTimeFormatResult icu4x_datetime_format_create_from_components_bag(const ICU4XLocale* locale, const ICU4XDataProvider* provider, const ICU4XComponentsBag* bag);

bool icu4x_datetime_format_write(const ICU4XDateTimeFormat* dtf, const ICU4XDateTime* value, ICU4XWriteable* write);

void icu4x_datetime_format_destroy(ICU4XDateTimeFormat* dtf);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_DATETIME_FORMAT_H
