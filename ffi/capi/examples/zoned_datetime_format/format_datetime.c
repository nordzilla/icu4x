// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/zoned_datetime_format.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    ICU4XLocale* locale = icu4x_locale_create("en", 2);
    ICU4XCreateDataProviderResult provider_result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!provider_result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }

    ICU4XDataProvider provider = provider_result.provider;

    ICU4XDateLength date = ICU4XDateLength_Full;
    ICU4XTimeLength time = ICU4XTimeLength_Full;

    ICU4XHourCycle hour_cycle = ICU4XHourCycle_H12;
    ICU4XPreferencesBag pref_bag = {
        .hour_cycle = &hour_cycle,
    };

    ICU4XLengthBag length_bag = {
        .date = &date,
        .time = &time,
        .preferences = &pref_bag,
    };

    ICU4XCreateZonedDateTimeFormatResult zdtf_result
        = icu4x_zoned_datetime_format_create_from_length_bag(locale, &provider, &length_bag);

    if (!zdtf_result.success) {
        printf("Failed to create First ZonedDateTimeFormat\n");
        return 1;
    }

    ICU4XZonedDateTimeFormat* zdtf = zdtf_result.zdtf;

    ICU4XDateTime dt = {
        .year = 2021,
        .month = 5,
        .day = 23,
        .hour = 15,
        .minute = 23,
        .second = 14,
    };

    ICU4XTimeZone tz = {
        .gmt_offset = -7 * 60 * 60,
        .time_zone_id = "America/Los_Angeles",
        .metazone_id = "America_Pacific",
        .time_variant = "daylight",
    };

    ICU4XZonedDateTime zdt = {
        .datetime = dt,
        .time_zone = tz,
    };

    char output[100];
    ICU4XWriteable write = icu4x_simple_writeable(output, 100);

    bool success = icu4x_zoned_datetime_format_write(zdtf, &zdt, &write);
    if (!success) {
        printf("Failed to write result of ZonedDateTimeFormat::format to string.\n");
        return 1;
    }
    printf("Formatted ZonedDateTime is %s\n", output);

    bool pass = true;
    const char* expected = u8"Thursday, June 24, 2021 at 3:23:14 PM Pacific Daylight Time";
    if (strcmp(output, expected) != 0) {
        pass = false;
        printf("Output does not match expected output: %s\n", expected);
    }


    ICU4XTimeLength time2 = ICU4XDateLength_Long;
    length_bag.time = &time2;

    ICU4XCreateZonedDateTimeFormatResult zdtf_result2
        = icu4x_zoned_datetime_format_create_from_length_bag(locale, &provider, &length_bag);

    if (!zdtf_result2.success) {
        printf("Failed to create Second ZonedDateTimeFormat\n");
        return 1;
    }

    ICU4XZonedDateTimeFormat* zdtf2 = zdtf_result2.zdtf;

    write = icu4x_simple_writeable(output, 60);

    success = icu4x_zoned_datetime_format_write(zdtf2, &zdt, &write);
    if (!success) {
        printf("Failed to write result of ZonedDateTimeFormat::format to string.\n");
        return 1;
    }
    printf("Formatted ZonedDateTime is %s\n", output);

    expected = u8"Thursday, June 24, 2021 at 3:23:14 PM PDT";
    if (strcmp(output, expected) != 0) {
        pass = false;
        printf("Output does not match expected output: %s\n", expected);
    }

    icu4x_zoned_datetime_format_destroy(zdtf);

    if (pass) {
        printf("[PASS]\n\n");
    }

    return 0;
}
