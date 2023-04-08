use crate::constants::{BASE_64_ENCODING_CHARS, PADDING_CHAR};
use std::str;

mod constants;

/**
 * The "base64" base encoding specified by <a
 * href="http://tools.ietf.org/html/rfc4648#section-4">RFC 4648 section 4</a>, Base 64 Encoding.
 * (This is the same as the base 64 encoding from <a
 * href="http://tools.ietf.org/html/rfc3548#section-3">RFC 3548</a>.)
 *
 * <p>The character {@code '='} is used for padding, but can be {@linkplain #omitPadding()
 * omitted} or {@linkplain #withPadChar(char) replaced}.
 *
 * <p>No line feeds are added by default, as per <a
 * href="http://tools.ietf.org/html/rfc4648#section-3.1">RFC 4648 section 3.1</a>, Line Feeds in
 * Encoded Data. Line feeds may be added using {@link #withSeparator(String, int)}.
 */

pub fn base64_encode(str: String) -> usize {
    let bytes = str.as_bytes();
    // base64_encode_bytes(bytes)
    return 0;
}

pub fn base64_encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let target_length = encode_calc_byte_size(bytes);
    let mut res:Vec<u8> = vec![0; target_length];
    let length = bytes.len();
    let mut position = 0;
    for i in 1..length {
        if i % 3 == 2 {
            let mut trio = [0; 3];
            trio[..3].clone_from_slice(&bytes[i - 2..i + 1]);
            let quartet = encode_trio(&trio);
            res[position..position + 4].clone_from_slice(&quartet);
            position += 4;
        }
    }
    let remaining = length % 3;
    if remaining > 0 {
        let mut remaining_bytes = vec![0; remaining];
        remaining_bytes[0..remaining].clone_from_slice(&bytes[length - remaining..length]);
        let quartet = if remaining == 2 { encode_duo(&remaining_bytes) } else { encode_uno(&remaining_bytes)};
        res[target_length - quartet.len()..target_length].clone_from_slice(&quartet);
    }
    return res.clone();
}

fn encode_calc_byte_size(bytes: &[u8]) -> usize {
    let res = (((bytes.len() as f32 * 4. / 3.) / 4.).ceil() * 4.);
    return res as usize;
}

fn encode_trio(bytes: &[u8]) -> [u8; 4] {
    assert_eq!(bytes.len(), 3);
    let quartet = bytes_encode_trio(bytes);
    return [
        BASE_64_ENCODING_CHARS[quartet[0]],
        BASE_64_ENCODING_CHARS[quartet[1]],
        BASE_64_ENCODING_CHARS[quartet[2]],
        BASE_64_ENCODING_CHARS[quartet[3]]
    ];
}

fn encode_duo(bytes: &[u8]) -> [u8; 4] {
    assert_eq!(bytes.len(), 2);
    let trio = [bytes[0], bytes[1], 63];
    let quartet = bytes_encode_trio(&trio);
    return [
        BASE_64_ENCODING_CHARS[quartet[0]],
        BASE_64_ENCODING_CHARS[quartet[1]],
        BASE_64_ENCODING_CHARS[quartet[2]],
        PADDING_CHAR
    ];
}

fn encode_uno(bytes: &[u8]) -> [u8; 4] {
    assert_eq!(bytes.len(), 1);
    let trio = [bytes[0], 15, 255];
    let quartet = bytes_encode_trio(&trio);
    return [
        BASE_64_ENCODING_CHARS[quartet[0]],
        BASE_64_ENCODING_CHARS[quartet[1]],
        PADDING_CHAR,
        PADDING_CHAR
    ];
}

fn bytes_encode_trio(bytes: &[u8]) -> [usize; 4] {
    let i = bytes[0];
    let first = i >> 2;
    let temp = (i & 3) << 4;
    let j = bytes[1];
    let second = (j >> 4) | temp;
    let temp1 = (j & 15) << 2;
    let k = bytes[2];
    let third = k >> 6 | temp1;
    let fourth = k & 63;
    return [first as usize, second as usize, third as usize, fourth as usize];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_encode_should_produce_right_results() {
        let bytes = "Man".as_bytes();
        let quartet = encode_trio(bytes);
        convert_to_str_check(&quartet, "TWFu");

        let duo = "Ma".as_bytes();
        let quartet_duo = encode_duo(duo);
        convert_to_str_check(&quartet_duo, "TWE=");

        let uno = "M".as_bytes();
        let quartet_uno = encode_uno(uno);
        convert_to_str_check(&quartet_uno, "TQ==");
    }

    fn convert_to_str_check(quartet: &[u8; 4], expected: &str) {
        let res = str::from_utf8(quartet);
        assert!(res.is_ok());
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn when_encode_calc_byte_size_should_produce_right_size() {
        let bytes = "Man".as_bytes();
        let res = encode_calc_byte_size(bytes);
        assert_eq!(4, res);
        let bytes2 = "Man1".as_bytes();
        let res = encode_calc_byte_size(bytes2);
        assert_eq!(8, res);
        let bytes3 = "Man12".as_bytes();
        let res = encode_calc_byte_size(bytes3);
        assert_eq!(8, res);
        let bytes3 = "Man1227".as_bytes();
        let res = encode_calc_byte_size(bytes3);
        assert_eq!(12, res);
    }

    #[test]
    fn when_base64_encode_bytes_should_produce_right_vector() {
        let input: Vec<&[u8]> = vec!["Man", "Assuming", "Olá! isto é um teste", "你好，这是一个测试"]
            .iter().map(|x| x.as_bytes()).collect();
        let output: Vec<&str> = vec!["TWFu", "QXNzdW1pbmc=", "T2zDoSEgaXN0byDDqSB1bSB0ZXN0ZQ==", "5L2g5aW977yM6L-Z5piv5LiA5Liq5rWL6K-V"];
        for (i, bytes) in input.iter().enumerate() {
            let vec = base64_encode_bytes(bytes);
            let res_str = str::from_utf8(&vec);
            assert_eq!(output[i], res_str.unwrap());
        }
    }
}
