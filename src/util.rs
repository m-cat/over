use std::fs::File;
use std::io;
use std::io::Write;

/// Writes a string to a file.
pub fn write_file_str(fname: &str, contents: &str) -> io::Result<()> {
    // Open a file in write-only mode
    let mut file = File::create(fname)?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
