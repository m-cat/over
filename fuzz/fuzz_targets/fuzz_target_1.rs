//! Simple fuzz test that makes sure no crashes occur in parsing.

#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate over;

use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        match over::Obj::from_str(s) {
            Ok(_) => (),
            Err(e) => {
                // Make sure all that error messages contain some information about where the error
                // occurred.
                assert!(format!("{}", e).contains("line"));
            },
        }
    }
});
