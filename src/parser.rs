//! Module containing parsing functions.

use object::Object;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct Parser {}

impl Parser {}

pub fn load_file(obj: &mut Object, path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {}

    Ok(())
}
