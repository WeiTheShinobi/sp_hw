#[derive(Debug, PartialEq)]
pub struct Header {
    len: u16,
    business_type: u8,
    format_code: u8,
    format_version: u8,
    number: usize,
}

pub fn parse_header(b: &[u8]) -> Header {
    Header {
        len: to_usize(&b[1..=2]) as u16,
        business_type: parse(b[3]),
        format_code: parse(b[4]),
        format_version: parse(b[5]),
        number: to_usize(&b[6..10]),
    }
}

pub fn to_usize(b: &[u8]) -> usize {
    let n = (1..b.len()).fold(1usize, |acc, _| acc * 100);

    b.iter().fold((0usize, n), |(result, n), &x| {
        (result + parse(x) as usize * n, n / 100)
    }).0
}

pub fn to_f32(data: &[u8], int_len: usize) -> f32 {
    let init_int_pos = (1..int_len).fold(1f32, |acc, _| acc * 100.0);

    let int_part = (0..int_len).
        fold((0f32, init_int_pos), |(result, n), i| {
            (result + parse(data[i]) as f32 * n, n / 100.0)
        }).0;

    let float_part = (int_len..data.len()).
        fold((0f32, 0.01), |(result, n), i| {
            (result + parse(data[i]) as f32 * n, n / 100.0)
        }).0;

    int_part + float_part
}

pub fn parse(v: u8) -> u8 {
    10 * (v >> 4) + (v & 0b00001111)
}

#[test]
fn test_parse_header() {
    let testcase: &[u8] = &[27, 1, 34, 1, 6, 4, 1, 0, 0, 1];
    let wanted = Header{
        len: 122,
        business_type: 1,
        format_code: 6,
        format_version: 4,
        number: 1000001,
    };

    assert_eq!(parse_header(testcase), wanted);
}


#[test]
fn test_to_usize() {
    assert_eq!(to_usize(&[0x01]), 1);
    assert_eq!(to_usize(&[0x11, 0x01]), 1101);
    assert_eq!(to_usize(&[0x00, 0x00, 0x01]), 1);
    assert_eq!(to_usize(&[0x35, 0x44, 0x00]), 354400);
    assert_eq!(to_usize(&[0x66, 0x52, 0x33, 0x11, 0x00, 0x00]), 665233110000);
}

#[test]
fn test_to_f32() {
    assert_eq!(to_f32(&[0x01, 0x01, 0x01], 1), 1.0101);
    assert_eq!(to_f32(&[0x11, 0x01, 0x00, 0x00, 0x01], 2), 1101.000001);
    assert_eq!(to_f32(&[0x00, 0x00, 0x01, 0x58, 0x43], 3), 1.5843);
}