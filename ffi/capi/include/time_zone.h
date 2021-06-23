// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_TIME_ZONE_H
#define ICU4X_TIME_ZONE_H

#include  <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    // GMT offset in seconds
    int32_t gmt_offset;
    char* time_zone_id;
    char* metazone_id;
    char* time_variant;
} ICU4XTimeZone;


#ifdef __cplusplus
}
#endif

#endif // ICU4X_TIME_ZONE_FORMAT_H
