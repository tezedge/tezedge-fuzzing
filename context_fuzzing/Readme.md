# context_fuzzing

Differential fuzzing of Tezedge context against Irmin implementation

## Execute

```sh
cargo +nightly run --release --bin context_fuzzing fuzz
```

## Replay a crash

Search for the last seed value printed on the screen and provide it to the fuzz sub command

```sh
cargo +nightly run --release --bin context_fuzzing replay 1337
```

## Documentation

The following section describe how the `context_fuzzing` fuzzer is working

### CLI Options

2 modes are available for now:
- fuzz: Run the Fuzzer
- replay: Replay the fuzzing session for a given seed

### Fuzzer

The fuzzer used a simple but efficient shift-xor algorithm for pseudo-random number generation (`Fuzzer::rand`)

The fuzzer structure contains multiple fields that will be initialize later in the fuzzer. Fields `keys` and `values` can be modified 

### context implementations

2 context implementation are tested:
- Irmin
- Texedge

Calls to Tezos Irmin library is done using Irmin binding (over Rust FFI calls)
Tezedge implementation are in pure Rust

### Fuzzer initialization

The fuzzer is initialized (line 164) with a seed, a list of interesting keys and a list of interesting values.

### Fuzzer behavior

The fuzzer is pretty simple, it's an infinite loop that, at each iteration, will generate a random sequence of byte. This sequence of bytes will later represent a list of action to execute.

Each byte of the list are them matched/compared to the defined context action ID.

The fuzzer will loop over this list of byte and at the same time execute the matching context action against both context implementation (irmin and tezedge)

### Bug detection 

After each context action execution, implementation results (like context hashes) are compare together and a panic is triggered if result is different.