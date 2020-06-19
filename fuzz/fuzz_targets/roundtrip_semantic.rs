
extern crate miniscript;

use std::str::FromStr;
use miniscript::{policy, DummyKey};

type DummyPolicy = policy::Semantic<DummyKey>;

fn do_test(data: &[u8]) {
    let data_str = String::from_utf8_lossy(data);
    if let Ok(pol) = DummyPolicy::from_str(&data_str) {
        let output = pol.to_string();
        assert_eq!(data_str.to_lowercase(), output.to_lowercase());
    }
}

#[cfg(feature = "afl")]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    afl::read_stdio_bytes(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}