# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard ../../include/*.h)
ALL_RUST := $(wildcard ../../src/*.rs ../../src/datetime_format/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):


../../../../target/debug/libicu_capi.a: $(ALL_RUST)
	cargo build

create_from_length_bag: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) create_from_length_bag.c
	gcc create_from_length_bag.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g -o create_from_length_bag

create_from_components_bag: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) create_from_components_bag.c
	gcc create_from_components_bag.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g -o create_from_components_bag

format_datetime: ../../../../target/debug/libicu_capi.a $(ALL_HEADERS) format_datetime.c
	gcc format_datetime.c ../../../../target/debug/libicu_capi.a -ldl -lpthread -lm -g -o format_datetime

build: create_from_length_bag create_from_components_bag format_datetime

clean:
	rm -rf a.out create_from_length_bag create_from_components_bag format_datetime

test: build
	./create_from_length_bag
	./create_from_components_bag
	./format_datetime