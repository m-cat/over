use std::{
    fs::File,
    io::{self, Write},
};

#[allow(unused_macros)]
macro_rules! map {
    { } => {
        ::std::collections::BTreeMap::new()
    };
    { $( $key:expr => $value:expr ),+ , } => {
        // Rule with trailing comma.
        map!{ $( $key => $value),+ }
    };
    { $( $key:expr => $value:expr ),* } => {
        {
            let mut _map = ::std::collections::BTreeMap::new();

            $(
                let _ = _map.insert($key, $value);
            )*

            _map
        }
    }
}

/// Writes a string to a file.
pub fn write_file_str(fname: &str, contents: &str) -> io::Result<()> {
    // Open a file in write-only mode
    let mut file = File::create(fname)?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
