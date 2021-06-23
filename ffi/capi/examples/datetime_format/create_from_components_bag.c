// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/datetime_format.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    ICU4XLocale* locale = icu4x_locale_create("bn", 2);
    ICU4XCreateDataProviderResult provider_result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!provider_result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }

    ICU4XDataProvider provider = provider_result.provider;

    ICU4XNumeric year = ICU4XNumeric_Numeric;
    ICU4XMonth month = ICU4XMonth_Narrow;
    ICU4XNumeric day = ICU4XNumeric_TwoDigit;
    ICU4XText weekday = ICU4XText_Short;

    ICU4XNumeric hour = ICU4XNumeric_TwoDigit;
    ICU4XNumeric minute = ICU4XNumeric_TwoDigit;
    ICU4XNumeric second = ICU4XNumeric_Numeric;

    ICU4XComponentsBag components_bag = {
        .era = NULL,
        .year = &year,
        .month = &month,
        .day = &day,
        .weekday = &weekday,

        .hour = &hour,
        .minute = &minute,
        .second = &second,

        .time_zone_name = NULL,

        .preferences = NULL,
    };

    ICU4XCreateDateTimeFormatResult dtf_result
        = icu4x_datetime_format_create_from_components_bag(locale, &provider, &components_bag);

    if (!dtf_result.success) {
        printf("Failed to create DateTimeFormat\n");
        return 1;
    }

    ICU4XDateTimeFormat* dtf = dtf_result.dtf;
    icu4x_datetime_format_destroy(dtf);

    printf("[PASS]\n\n");

    return 0;
}
