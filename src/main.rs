use std::fs::File;
use std::io;
use std::io::Read;

const FILE_NAME: &str = "f6_01000001_01001000_TP03.new";

fn main() -> io::Result<()> {
    let mut file = File::open(FILE_NAME)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("File contents as bytes: {:?}", buffer);

    Ok(())
}

