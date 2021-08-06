.PHONY: run test list

# $(a) is the target
# to use it: make run a=BinaryChunk_from_content
default: run

# -T: Use SIGVTALRM to kill timeouting processes (i.e. convert timeout to crashes)
# -t : Timeout in seconds
HFUZZ_RUN_ARGS := HFUZZ_RUN_ARGS="-T -t 60"

# Tezos shared library path
TEZOS_SO_PATH := ../code/tezedge/tezos/sys/lib_tezos/artifacts

list:
	ls src/bin/ | awk '{print substr($$0, 1, length($$0)-3)}'

run: list
	$(call check_defined, a)
	LD_LIBRARY_PATH=$(TEZOS_SO_PATH) $(HFUZZ_RUN_ARGS) cargo hfuzz run $(a)

debug:
	$(call check_defined, a)
	$(call check_defined, crash)
	LD_LIBRARY_PATH=$(TEZOS_SO_PATH) $(HFUZZ_RUN_ARGS) cargo hfuzz run-debug $(a) $(crash)

debug-all:
	export LD_LIBRARY_PATH=.
	cargo hfuzz build-debug

diff_fuzz:
	cargo run --bin context_fuzzing fuzz

diff_replay:
	cargo run --bin context_fuzzing replay 1337

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
