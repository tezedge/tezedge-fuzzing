.PHONY: run test list

# $(a) is the target
# to use it: make run a=BinaryChunk_from_content
default: run

# -T: Use SIGVTALRM to kill timeouting processes (i.e. convert timeout to crashes)
# -t : Timeout in seconds
HFUZZ_RUN_ARGS := HFUZZ_RUN_ARGS="-T -t 10"


list:
	ls src/bin/ | awk '{print substr($$0, 1, length($$0)-3)}'

run: list
	$(call check_defined, a)
	$(HFUZZ_RUN_ARGS) cargo +nightly hfuzz run $(a)

debug:
	$(call check_defined, a)
	$(call check_defined, crash)
	$(HFUZZ_RUN_ARGS) cargo +nightly hfuzz run-debug $(a) $(crash)

debug-all:
	cargo +nightly hfuzz build-debug

parse_query_string:
	HFUZZ_RUN_ARGS="--only_printable" cargo +nightly hfuzz run parse_query_string

clean:
	rm -rf hfuzz_target









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