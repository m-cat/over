#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate over;

use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        match over::Obj::from_str(s) {
            Ok(o1) => {
                match over::Obj::from_str(&o1.write_str()) {
                    Ok(o2) => {
                        if o1 != o2 {
                            panic!(format!("Read object is different: {}", o2));
                        }
                    },
                    Err(e) => {
                        panic!(format!("{}", e));
                    },
                }
            },
            Err(e) => {
                // Make sure all that error messages contain some information about where the error
                // occurred.
                assert!(format!("{}", e).contains("line"));
            },
        }
    }
});
