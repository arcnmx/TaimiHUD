use std::env;

fn main() {
    #[cfg(feature = "built")]
    if env::var_os("CARGO_FEATURE_built").is_some() {
        built::write_built_file().expect("Failed to acquire build-time information");
    }
}
