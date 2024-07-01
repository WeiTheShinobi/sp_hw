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

struct data(f32, f32);

fn parse_chunk(chunk: &[u8]) {
    let code = chunk[10..16].iter().map(|&c| c as char).collect::<String>();
    let (have_deal_price, buy_price_count, sell_price_count, have_best_5) = parse_3dot3(&chunk);
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

fn parse_time(chunk: &[u8]) -> u64 {
    let hour = parse_bcd(chunk[0]) as u32;
    let minute = parse_bcd(chunk[1]) as u32;
    let second = parse_bcd(chunk[2]) as u32;

    let micro_b1 = parse_bcd(chunk[3]) as u32;
    let micro_b2 = parse_bcd(chunk[4]) as u32;
    let micro_b3 = parse_bcd(chunk[5]) as u32;
    let micro = micro_b1 * 10000 + micro_b2 * 100 + micro_b3;

    ((hour * 10000 + minute * 100 + second) as u64) * 1000000 + micro as u64
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time(&[0x23, 0x59, 0x59, 0x51, 0x01, 0x15]), 235959510115);
}

fn parse_price(chunk: &[u8]) -> f32 {
    let v1 = parse_bcd(chunk[0]) as f32;
    let v2 = parse_bcd(chunk[1]) as f32;
    let v3 = parse_bcd(chunk[2]) as f32;
    let v4 = parse_bcd(chunk[3]) as f32;
    let v5 = parse_bcd(chunk[4]) as f32;

    v1 * 10000f32 + v2 * 100f32 + v3 + v4 * 0.01 + v5 * 0.0001
}

#[test]
fn test_parse_price() {
    assert_eq!(parse_price(&[0x01, 0x00, 0x0, 0x0, 0x0]), 10000f32);
    assert_eq!(parse_price(&[0x01, 0x11, 0x11, 0x11, 0x11]), 11111.1111);
    assert_eq!(parse_price(&[0x01, 0x01, 0x90, 0x01, 0x00]), 10190.01);
    assert_eq!(parse_price(&[0x00, 0x00, 0x01, 0x00, 0x01]), 1.0001);
}

fn parse_amount(chunk: &[u8]) -> usize {
    let v1 = parse_bcd(chunk[0]) as usize;
    let v2 = parse_bcd(chunk[1]) as usize;
    let v3 = parse_bcd(chunk[2]) as usize;
    let v4 = parse_bcd(chunk[3]) as usize;
    v1 * 1000000 + v2 * 10000 + v3 * 100 + v4
}

#[test]
fn test_parse_amount() {
    assert_eq!(parse_amount(&[0x00, 0x00, 0x00, 0x10]), 10);
    assert_eq!(parse_amount(&[0x00, 0x00, 0x11, 0x10]), 1110);
    assert_eq!(parse_amount(&[0x00, 0x90, 0x11, 0x10]), 901110);
    assert_eq!(parse_amount(&[0x00, 0x01, 0x00, 0x01]), 10001);
}

fn parse_3dot3(chunk: &[u8]) -> (bool, u8, u8, bool) {
    let v = chunk[22];
    let have_deal_price = (v >> 7) & 1 == 1;
    let buy_price_count = (v >> 4) & 0b111;
    let sell_price_count = (v >> 1) & 0b111;
    let have_best_5 = v & 1 == 1;
    println!("{}, {}, {}, {}", have_deal_price, buy_price_count, sell_price_count, have_best_5);

    (have_deal_price, buy_price_count, sell_price_count, have_best_5)
}

#[test]
fn test_parse_3dot3() {
    assert_eq!(parse_3dot3(&[0b10000001u8; 23]), (true, 0, 0, true));
    assert_eq!(parse_3dot3(&[0b10010001u8; 23]), (true, 1, 0, true));
    assert_eq!(parse_3dot3(&[0b11011011u8; 23]), (true, 5, 5, true));
    assert_eq!(parse_3dot3(&[0b10010101u8; 23]), (true, 1, 2, true));
}

fn parse_msg_len(v: &[u8]) -> usize {
    (parse_bcd(v[1]) as usize) * 100 + parse_bcd(v[2]) as usize
}

fn parse_bcd(v: u8) -> u8 {
    10 * (v >> 4) + (v & 0b00001111)
}

#[test]
fn test_parse_bcd() {
    assert_eq!(parse_bcd(0x12), 12);
    assert_eq!(parse_bcd(0x93), 93);
    assert_eq!(parse_bcd(0x01), 1);
    assert_eq!(parse_bcd(0x10), 10);
}

#[test]
fn test_parse_msg_len() {
    assert_eq!(parse_msg_len(&[0, 0x12, 0x93]), 1293);
    assert_eq!(parse_msg_len(&[0, 0x01, 0x22]), 122);
    assert_eq!(parse_msg_len(&[0, 0x10, 0x01]), 1001);
    assert_eq!(parse_msg_len(&[0, 0x00, 0x01]), 1);
}
