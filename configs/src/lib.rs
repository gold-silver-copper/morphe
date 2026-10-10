//! Language descriptions. Adding a language means adding a module here.

pub mod english;
pub mod russian;

use morphe_train::spec::Language;

pub fn all() -> Vec<Box<dyn Language>> {
    vec![Box::new(english::English), Box::new(russian::Russian)]
}
