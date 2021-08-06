use honggfuzz::fuzz;

fn main() {
    let mut fuzz_target = context_fuzzing::workingtree_deserialize_serialize_fuzz();
    loop {
        fuzz!(|data: &[u8]| {
            fuzz_target(data);
        });
    }
}
