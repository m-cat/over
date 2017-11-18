use std::fs::File;
use std::io;
use std::io::Write;

macro_rules! map {
    { } => {
        ::std::collections::HashMap::new()
    };
    { $( $key:expr => $value:expr ),+ , } => {
        // Rule with trailing comma.
        map!{ $( $key => $value),+ }
    };
    { $( $key:expr => $value:expr ),* } => {
        {
            let mut _map = ::std::collections::HashMap::new();

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

/// Returns true if `ch` is an ASCII decimal digit.
pub fn is_digit(ch: char) -> bool {
    match ch {
        '0'...'9' => true,
        _ => false,
    }
}
