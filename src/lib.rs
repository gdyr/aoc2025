use std::{fs::File, io::{self, BufRead}, path::Path};

#[allow(clippy::missing_errors_doc)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}