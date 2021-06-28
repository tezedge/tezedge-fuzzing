.PHONY: list build run dry_run dry_run_all

default: run

list:
	ls fuzz/fuzz_targets/ | awk '{print substr($$0, 1, length($$0)-3)}'

build:
	cargo +nightly build

run: list
	$(call check_defined, a)
	cargo +nightly fuzz run $(a)

run_all: list
	$(call check_defined, a)
	cargo +nightly fuzz run $(a) -- -runs=10000000

dry_run: list
	$(call check_defined, a)
	cargo +nightly fuzz run $(a) fuzz/corpus/$(a)/* || exit 1;

dry_run_all:
	for target in `ls fuzz/fuzz_targets/ | awk '{print substr($$0, 1, length($$0)-3)}')` ; do \
		echo $$target ; \
		cargo +nightly fuzz run $$target fuzz/corpus/$$target/* || exit 1; \
	done























# Check that given variables are set and all have non-empty values,
# die with an error otherwise.
#
# Params:
#   1. Variable name(s) to test.
#   2. (optional) Error message to print.
check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))