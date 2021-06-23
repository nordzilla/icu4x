// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/datetime_format.h"
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

    ICU4XDateLength date = ICU4XDateLength_Long;
    ICU4XTimeLength time = ICU4XTimeLength_Short;

    ICU4XHourCycle hour_cycle = ICU4XHourCycle_H12;
    ICU4XPreferencesBag pref_bag = {
        .hour_cycle = &hour_cycle,
    };

    ICU4XLengthBag length_bag = {
        .date = &date,
        .time = &time,
        .preferences = &pref_bag,
    };

    ICU4XCreateDateTimeFormatResult dtf_result
        = icu4x_datetime_format_create_from_length_bag(locale, &provider, &length_bag);

    if (!dtf_result.success) {
        printf("Failed to create DateTimeFormat\n");
        return 1;
    }

    ICU4XDateTimeFormat* dtf = dtf_result.dtf;

    ICU4XDateTime dt = {
        .year = 2021,
        .month = 5,
        .day = 22,
        .hour = 16,
        .minute = 50,
        .second = 0,
    };

    char output[40];
    ICU4XWriteable write = icu4x_simple_writeable(output, 40);

    bool success = icu4x_datetime_format_write(dtf, &dt, &write);
    if (!success) {
        printf("Failed to write result of DateTimeFormat::format to string.\n");
        return 1;
    }
    printf("Formatted DateTime is %s\n", output);

    const char* expected = "June 23, 2021 at 4:50 PM";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    icu4x_datetime_format_destroy(dtf);

    printf("[PASS]\n\n");

    return 0;
}
