export LD_LIBRARY_PATH=.

.PHONY: all
	
# $(a) is the target
# to use it: make run a=BinaryChunk_from_content
default: run

# -T: Use SIGVTALRM to kill timeouting processes (i.e. convert timeout to crashes)
# -t : Timeout in seconds
HFUZZ_RUN_ARGS := HFUZZ_RUN_ARGS="-T -t 60"


list:
	ls src/bin/ | awk '{print substr($$0, 1, length($$0)-3)}'

run: list
	$(call check_defined, a)
	# export LD_LIBRARY_PATH=.
	$(HFUZZ_RUN_ARGS) cargo +nightly hfuzz run $(a)

debug:
	$(call check_defined, a)
	$(call check_defined, crash)
	$(HFUZZ_RUN_ARGS) cargo +nightly hfuzz run-debug $(a) $(crash)

debug-all:
	# export LD_LIBRARY_PATH=.
	cargo +nightly hfuzz build-debug

diff_fuzz:
	cargo +nightly run --release --bin context_fuzzing fuzz

diff_replay:
	cargo +nightly run --release --bin context_fuzzing replay 1337

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