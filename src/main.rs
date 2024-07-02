mod bcd;

use std::fs::File;
use std::io;
use std::io::Read;

const FILE_NAME: &str = "f6_01000001_01001000_TP03.new";

fn main() -> io::Result<()> {
    let mut file = File::open(FILE_NAME)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut result = Vec::new();
    let mut i = 0usize;

    while i < buffer.len() {
        let len = bcd::parse_header(&buffer[i..i + 10]).len as usize;
        let chunk = &buffer[i..i + len];
        if let Some(d) = parse_chunk(chunk) {
            result.push(d);
        }
        i += len;
    }

    result.iter().for_each(|d| println!("{:?}", d));
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Data {
    code: String,
    bid_price: f32,
    ask_price: f32,
    // HHMMSSnnnnnn 6nano second
    time: usize,
}


const CODE_START_INDEX: usize = 10;
const CODE_END_INDEX: usize = 16;
const TIME_START_INDEX: usize = 16;
const TIME_END_INDEX: usize = 22;
const PRICE_START_INDEX: usize = 29;
const DOC3_3_INDEX: usize = 22;
const DOC3_7_LEN: usize = 9;
const DOC3_7_PRICE_LEN: usize = 5;


fn parse_chunk(chunk: &[u8]) -> Option<Data> {
    let code = chunk[CODE_START_INDEX..CODE_END_INDEX].iter().map(|&c| c as char).collect::<String>();
    let time = parse_time(&chunk[TIME_START_INDEX..TIME_END_INDEX]);
    if time == 0 {
        return None;
    }
    let (have_deal_price, buy_price_count, sell_price_count, _) = parse_3_3(chunk[DOC3_3_INDEX]);

    let mut index = PRICE_START_INDEX + if have_deal_price { DOC3_7_LEN } else { 0 };
    let bid_price = 0f32 + if buy_price_count > 0 {
        parse_price(&chunk[index..index + DOC3_7_PRICE_LEN])
    } else {
        0f32
    };

    index += buy_price_count as usize * DOC3_7_LEN;

    let ask_price = 0f32 + if sell_price_count > 0 {
        parse_price(&chunk[index..index + DOC3_7_PRICE_LEN])
    } else {
        0f32
    };

    Some(Data {
        code,
        bid_price,
        ask_price,
        time,
    })
}

#[test]
fn test_parse_chunk() {
    let testcase: &[u8] = &[27, 1, 34, 1, 6, 4, 1, 0, 0, 1, 48, 48, 54, 55, 53, 76, 9, 9,
        52, 68, 118, 152, 90, 0, 16, 0, 0, 2, 34, 0, 0, 64, 97, 0, 0, 0, 0, 88, 0, 0, 64,
        96, 0, 0, 0, 0, 83, 0, 0, 64, 86, 0, 0, 0, 0, 96, 0, 0, 64, 82, 0, 0, 0, 1, 0, 0,
        0, 64, 80, 0, 0, 0, 0, 6, 0, 0, 64, 113, 0, 0, 0, 0, 73, 0, 0, 64, 114, 0, 0, 0,
        1, 70, 0, 0, 64, 115, 0, 0, 0, 1, 72, 0, 0, 64, 118, 0, 0, 0, 0, 1, 0, 0, 64, 119,
        0, 0, 0, 0, 5, 167, 13, 10];

    let result = parse_chunk(testcase);
    println!("parse_chunk: {:?}", result)
}

fn parse_time(chunk: &[u8]) -> usize {
    bcd::to_usize(&chunk)
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time(&[0x23, 0x59, 0x59, 0x51, 0x01, 0x15]), 235959510115);
}

fn parse_price(chunk: &[u8]) -> f32 {
    bcd::to_f32(chunk, 3)
}

#[test]
fn test_parse_price() {
    assert_eq!(parse_price(&[0x01, 0x00, 0x0, 0x0, 0x0]), 10000f32);
    assert_eq!(parse_price(&[0x01, 0x11, 0x11, 0x11, 0x11]), 11111.1111);
    assert_eq!(parse_price(&[0x01, 0x01, 0x90, 0x01, 0x00]), 10190.01);
    assert_eq!(parse_price(&[0x00, 0x00, 0x01, 0x00, 0x01]), 1.0001);
}

fn parse_amount(chunk: &[u8]) -> usize {
    bcd::to_usize(chunk)
}

#[test]
fn test_parse_amount() {
    assert_eq!(parse_amount(&[0x00, 0x00, 0x00, 0x10]), 10);
    assert_eq!(parse_amount(&[0x00, 0x00, 0x11, 0x10]), 1110);
    assert_eq!(parse_amount(&[0x00, 0x90, 0x11, 0x10]), 901110);
    assert_eq!(parse_amount(&[0x00, 0x01, 0x00, 0x01]), 10001);
}

fn parse_3_3(v: u8) -> (bool, u8, u8, bool) {
    let have_deal_price = (v >> 7) & 1 == 1;
    let buy_price_count = (v >> 4) & 0b111;
    let sell_price_count = (v >> 1) & 0b111;
    let have_best_5 = v & 1 == 1;

    (have_deal_price, buy_price_count, sell_price_count, have_best_5)
}

#[test]
fn test_parse_3_3() {
    assert_eq!(parse_3_3(0b10000001), (true, 0, 0, true));
    assert_eq!(parse_3_3(0b10010001), (true, 1, 0, true));
    assert_eq!(parse_3_3(0b11011011), (true, 5, 5, true));
    assert_eq!(parse_3_3(0b10010101), (true, 1, 2, true));
}
