// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_ZONED_DATETIME_H
#define ICU4X_ZONED_DATETIME_H

#include  <stdint.h>
#include "datetime.h"
#include "time_zone.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    ICU4XDateTime datetime;
    ICU4XTimeZone time_zone;
} ICU4XZonedDateTime;


#ifdef __cplusplus
}
#endif

#endif // ICU4X_ZONED_DATETIME_FORMAT_H
