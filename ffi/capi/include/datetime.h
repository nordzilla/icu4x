// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_DATETIME_H
#define ICU4X_DATETIME_H

#include  <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    /// ISO-8601 year (proleptic Gregorian).
    int32_t year;

    /// 0-based month index.
    uint32_t month;

    /// 0-based day index.
    uint32_t day;

    /// 0-based hour.
    uint8_t hour;

    /// 0-based minute.
    uint8_t minute;

    /// 0-based second.
    uint8_t second;
} ICU4XDateTime;


#ifdef __cplusplus
}
#endif

#endif // ICU4X_DATETIME_FORMAT_H
