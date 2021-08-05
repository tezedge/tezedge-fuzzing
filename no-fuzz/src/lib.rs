
/// Apply persisted fuzzing data to a function.
///
/// Fuzzing data (corpus) should be located under a directory specified
/// in `CORPUS` environment variable, inside its subdirectory named [target].
/// If `CORPUS` variable is not set, then `hfuzz_workspace/[target]/input`
/// directory is used.
pub fn no_fuzz(target: &str, mut f: impl FnMut(&[u8])) {
    let corpus = match std::env::var("CORPUS") {
        Ok(v) => format!("{}/{}", v, target),
        Err(_) => format!("hfuzz_workspace/{}/input", target),
    };
    eprintln!("{}", corpus);
    for e in std::fs::read_dir(corpus).expect("cannot read directory") {
        let path = e.expect("cannot get dir entry").path();
        if path.is_file() {
            let mut o = std::fs::File::open(path).expect("cannot read file");
            let mut d = vec![];
            use std::io::Read;
            o.read_to_end(&mut d).expect("cannot read file");
            f(&d);
        }
    }
}

#[macro_export]
macro_rules! no_fuzz {
    ($name:ident, $target:expr) => {
        #[test]
        fn $name() {
            no_fuzz(stringify!($name), $target);
        }
	};
}

#[macro_export]
macro_rules! hfuzz {
    ($target:expr) => {
        #[allow(non_snake_case)]
        fn main() {
            loop {
                honggfuzz::fuzz!(|data: &[u8]| {
                    $target(&data);
                });
            }
        }
    };
}
