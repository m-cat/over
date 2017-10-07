mod error;
mod parser;

use {Obj, OverError, OverResult};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn load_file(obj: &mut Obj, path: &str) -> OverResult<()> {
    let file = File::open(path).map_err(OverError::from)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {}

    Ok(())
}

pub fn write_to_file(obj: &Obj, path: &str) -> OverResult<()> {
    let file = File::open(path).map_err(OverError::from)?;

    Ok(())
}
