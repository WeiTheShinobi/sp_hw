use std::fs::File;
use std::io;
use std::io::Read;

const FILE_NAME: &str = "f6_01000001_01001000_TP03.new";

fn main() -> io::Result<()> {
    let mut file = File::open(FILE_NAME)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(())
}

fn parse_msg_len(v: &[u8]) -> usize {
    (parse_bcd(v[0]) as usize) * 100 + parse_bcd(v[1]) as usize
}

fn parse_bcd(v: u8) -> u8 {
    10 * (v >> 4) + (v & 0b00001111)
}

#[test]
fn test_parse_bcd() {
    assert_eq!(parse_bcd(0b00010010), 12);
    assert_eq!(parse_bcd(0b10010011), 93);
    assert_eq!(parse_bcd(0b00000001), 1);
    assert_eq!(parse_bcd(0b00010000), 10);
}

#[test]
fn test_parse_msg_len() {
    assert_eq!(parse_msg_len(&[0b00010010, 0b10010011]), 1293);
    assert_eq!(parse_msg_len(&[0b00000001, 0b00100010]), 122);
    assert_eq!(parse_msg_len(&[0b00010000, 0b00000001]), 1001);
    assert_eq!(parse_msg_len(&[0b00000000, 0b00000001]), 1);
}
