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
        if let Some(d) = parse_6_6_body(chunk) {
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


const STOCK_CODE_INDEX: (usize, usize) = (10, 16);
const TIME_INDEX: (usize, usize) = (16, 22);
const PRICE_START_INDEX: usize = 29;
const DOC3_3_INDEX: usize = 22;
const DOC3_7_LEN: usize = 9;
const DOC3_7_PRICE_LEN: usize = 5;


fn parse_6_6_body(chunk: &[u8]) -> Option<Data> {
    let code = chunk[STOCK_CODE_INDEX.0..STOCK_CODE_INDEX.1].iter().map(|&c| c as char).collect::<String>();
    let time = bcd::to_usize(&chunk[TIME_INDEX.0..TIME_INDEX.1]);
    if time == 0 {
        return None;
    }
    let data_3_3 = parse_3_3(chunk[DOC3_3_INDEX]);

    let mut index = PRICE_START_INDEX + if data_3_3.have_deal_price { DOC3_7_LEN } else { 0 };
    let bid_price = 0f32 + if data_3_3.buy_price_count > 0 {
        bcd::to_f32(&chunk[index..index + DOC3_7_PRICE_LEN], 3)
    } else {
        0f32
    };

    index += data_3_3.buy_price_count as usize * DOC3_7_LEN;

    let ask_price = 0f32 + if data_3_3.sell_price_count > 0 {
        bcd::to_f32(&chunk[index..index + DOC3_7_PRICE_LEN], 3)
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

    let result = parse_6_6_body(testcase);
    println!("parse_chunk: {:?}", result)
}

#[derive(Debug, PartialEq)]
struct Data3d3 {
    have_deal_price: bool,
    buy_price_count: u8,
    sell_price_count: u8,
    have_best_5: bool,
}

fn parse_3_3(v: u8) -> Data3d3 {
    Data3d3 {
        have_deal_price: (v >> 7) & 1 == 1,
        buy_price_count: (v >> 4) & 0b111,
        sell_price_count: (v >> 1) & 0b111,
        have_best_5: v & 1 == 1,
    }
}

#[test]
fn test_parse_3_3() {
    assert_eq!(parse_3_3(0b10000001), Data3d3 {
        have_deal_price: true,
        buy_price_count: 0,
        sell_price_count: 0,
        have_best_5: true,
    });
    assert_eq!(parse_3_3(0b10010001), Data3d3 {
        have_deal_price: true,
        buy_price_count: 1,
        sell_price_count: 0,
        have_best_5: true,
    });
    assert_eq!(parse_3_3(0b11011011), Data3d3 {
        have_deal_price: true,
        buy_price_count: 5,
        sell_price_count: 5,
        have_best_5: true,
    });
    assert_eq!(parse_3_3(0b00010101), Data3d3 {
        have_deal_price: false,
        buy_price_count: 1,
        sell_price_count: 2,
        have_best_5: true,
    });
}
