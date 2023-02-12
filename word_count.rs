use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("Expected a filename");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let word_count: u32 = reader.lines().flat_map(|l| l.ok()).map(|l| l.split_whitespace().count() as u32).sum();
    println!("{}", word_count);
    Ok(())
}
