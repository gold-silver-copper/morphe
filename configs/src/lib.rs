//! Language descriptions. Adding a language means adding `src/langs/<name>.rs`
//! with `pub fn language() -> Box<dyn Language>`; it is picked up automatically.

include!(concat!(env!("OUT_DIR"), "/langs.rs"));
