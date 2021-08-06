use honggfuzz::fuzz;

fn main() {
    loop {
        let mut fuzz_target = context_fuzzing::context_api_fuzz();
        fuzz!(|data| {
            fuzz_target(data)
        });
    }
}
