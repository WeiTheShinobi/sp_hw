use std::fs::File;
use std::io;
use std::io::Read;

const FILE_NAME: &str = "f6_01000001_01001000_TP03.new";

fn main() -> io::Result<()> {
    let mut file = File::open(FILE_NAME)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", &buffer[..122]);
    println!("{:?}", parse_msg_len(&buffer[..122]));

    Ok(())
}

fn parse_chunk(chunk: &[u8]) {
    let code = chunk[10..16].iter().map(|&c| c as char).collect::<String>();
    println!("{:?}", code);
}

#[test]
fn test_parse_chunk() {
    let testcase: &[u8] = &[27, 1, 34, 1, 6, 4, 1, 0, 0, 1, 48, 48, 54, 55, 53, 76, 9, 9,
        52, 68, 118, 152, 90, 0, 16, 0, 0, 2, 34, 0, 0, 64, 97, 0, 0, 0, 0, 88, 0, 0, 64,
        96, 0, 0, 0, 0, 83, 0, 0, 64, 86, 0, 0, 0, 0, 96, 0, 0, 64, 82, 0, 0, 0, 1, 0, 0,
        0, 64, 80, 0, 0, 0, 0, 6, 0, 0, 64, 113, 0, 0, 0, 0, 73, 0, 0, 64, 114, 0, 0, 0,
        1, 70, 0, 0, 64, 115, 0, 0, 0, 1, 72, 0, 0, 64, 118, 0, 0, 0, 0, 1, 0, 0, 64, 119,
        0, 0, 0, 0, 5, 167, 13, 10];

    parse_chunk(testcase)
}

fn parse_3dot3(chunk: &[u8]) -> (bool, u8, u8, bool) {
    let v = chunk[22];
    let have_deal_price = (v >> 7) & 1 == 1;
    let buy_count = (v >> 4) & 0b111;
    let sell_count = (v >> 1) & 0b111;
    let have_best_5 = v & 1 == 1;
    println!("{}, {}, {}, {}", have_deal_price, buy_count, sell_count, have_best_5);

    (have_deal_price, buy_count, sell_count, have_best_5)
}

#[test]
fn test_parse_3dot3() {
    let testcase = &[0b10000001u8; 23];
    assert_eq!(parse_3dot3(testcase), (true, 0, 0, true));
    let testcase = &[0b10010001u8; 23];
    assert_eq!(parse_3dot3(testcase), (true, 1, 0, true));
    let testcase = &[0b11011011u8; 23];
    assert_eq!(parse_3dot3(testcase), (true, 5, 5, true));
    let testcase = &[0b10010101u8; 23];
    assert_eq!(parse_3dot3(testcase), (true, 1, 2, true));
}

fn parse_msg_len(v: &[u8]) -> usize {
    (parse_bcd(v[1]) as usize) * 100 + parse_bcd(v[2]) as usize
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
    assert_eq!(parse_msg_len(&[0, 0b00010010, 0b10010011]), 1293);
    assert_eq!(parse_msg_len(&[0, 0b00000001, 0b00100010]), 122);
    assert_eq!(parse_msg_len(&[0, 0b00010000, 0b00000001]), 1001);
    assert_eq!(parse_msg_len(&[0, 0b00000000, 0b00000001]), 1);
}
