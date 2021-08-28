use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn read_file(filename: &Path) -> std::io::Result<String> {
    println!("{}", filename.display());
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename = Path::new("input.txt");
    let read_result: std::io::Result<String> = read_file(filename);
    match read_result {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    }
}
