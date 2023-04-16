use std::{fs, str};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str::Utf8Error;

use crate::constants::{BASE_64_ENCODING_CHARS, CHARS_BASE_64_ENCODING, PADDING_CHAR};
use crate::errors::{Base64Error, PaddingError};

mod constants;
mod errors;

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

pub fn base64_encode(str: String) -> Result<String, Utf8Error> {
    let bytes = str.as_bytes();
    let vec = base64_encode_bytes(bytes);
    let res = str::from_utf8(&vec)?;
    return Ok(res.to_string());
}

pub fn base64_decode(str: String) -> Result<String, Base64Error> {
    let bytes = str.as_bytes();
    let decoded_result = base64_decode_bytes(bytes);
    match decoded_result {
        Ok(decoded) => {
            match str::from_utf8(&decoded) {
                Ok(s) => { Ok(s.to_string()) }
                Err(error) => {
                    Err(Base64Error { msg: "UTF8 encoding failed".to_string(), utf8_error: Some(error) })
                }
            }
        }
        Err(_) => {
            Err(Base64Error { msg: "Decoding failed".to_string(), utf8_error: None })
        }
    }
}

pub fn base64_encode_file_str(path_str: &str) -> Result<Vec<u8>, Error> {
    let path = PathBuf::from(path_str);
    base64_encode_file(path)
}

pub fn base64_encode_file(path: PathBuf) -> Result<Vec<u8>, Error> {
    let data = fs::read(path)?;
    let encoded = base64_encode_bytes(&data);
    Ok(encoded)
}

pub fn base64_encode_to_file(path: PathBuf, target_path: PathBuf) -> Result<usize, Error> {
    let res = base64_encode_file(path)?;
    let len = res.len();
    fs::write(target_path, res)?;
    Ok(len)
}

pub fn base64_decode_from_file(source_path: PathBuf, target_path: PathBuf) -> Result<usize, Error> {
    let data = fs::read(source_path)?;
    let bytes = data.as_slice();
    let decoded_res = base64_decode_bytes(bytes);
    match decoded_res {
        Ok(decoded) => {
            let len = decoded.len();
            fs::write(target_path, decoded)?;
            Ok(len)
        }
        Err(_) => {
            Err(std::io::Error::new(ErrorKind::InvalidInput, "Padding error occurred."))
        }
    }
    
}

pub fn base64_encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let target_length = encode_calc_byte_size(bytes);
    let mut res: Vec<u8> = vec![0; target_length];
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
        let quartet = if remaining == 2 { encode_duo(&remaining_bytes) } else { encode_uno(&remaining_bytes) };
        res[target_length - quartet.len()..target_length].clone_from_slice(&quartet);
    }
    res.clone()
}

pub fn base64_decode_bytes(bytes: &[u8]) -> Result<Vec<u8>, PaddingError> {
    let target_length = decode_calc_byte_size(bytes);
    let mut res = vec![0; target_length];
    let source_length = bytes.len();
    const CHUNK: usize = 4;
    let modulo_max = CHUNK - 1;
    let mut position = 0;
    for i in 1..source_length - CHUNK {
        if i % CHUNK == modulo_max {
            let converted = convert_encoded_bytes(&bytes[i - modulo_max..i + 1]);
            let decoded = decode_quartet(&converted);
            res[position..position + 3].clone_from_slice(&decoded);
            position += 3;
        }
    }
    let converted = convert_encoded_bytes(&bytes[(source_length - CHUNK)..source_length]);
    let decoded = decode_incomplete(&converted)?;
    res[target_length - decoded.len()..target_length].clone_from_slice(&decoded[0..decoded.len()]);
    Ok(res)
}

fn encode_calc_byte_size(bytes: &[u8]) -> usize {
    let res = ((bytes.len() as f32 * 4. / 3.) / 4.).ceil() * 4.;
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

fn decode_calc_byte_size(bytes: &[u8]) -> usize {
    let real_length = bytes.iter().position(|&r| r == '=' as u8).unwrap_or(bytes.len());
    (real_length as f32 * 3. / 4.).floor() as usize
}

fn convert_encoded_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|x| CHARS_BASE_64_ENCODING[*x as usize]).collect()
}

fn decode_incomplete(bytes: &[u8]) -> Result<Vec<u8>, PaddingError> {
    let mut quartet: [u8; 4] = [0; 4];
    let pad_code = CHARS_BASE_64_ENCODING['=' as usize];
    let pad_pos = bytes.iter().position(|&r| r == pad_code).unwrap_or(bytes.len());
    quartet[0..pad_pos].clone_from_slice(&bytes[0..pad_pos]);
    let temp = decode_quartet(&quartet);
    match pad_pos {
        4 => Ok(vec![temp[0], temp[1], temp[2]]),
        3 => Ok(vec![temp[0], temp[1]]), // one =
        2 => Ok(vec![temp[0]]), // two =
        _ => Err(PaddingError {}) // something wrong
    }
}

fn decode_quartet(bytes: &[u8]) -> [u8; 3] {
    let i = bytes[0];
    let j = bytes[1];
    let k = bytes[2];
    let l = bytes[3];
    let first = (i << 2) | (j >> 4);
    let second = (j << 4) | (k >> 2) & 0xff;
    let third = ((k << 6) | l) & 0xff;
    return [first as u8, second as u8, third as u8];
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
        let output: Vec<&str> = vec!["TWFu", "QXNzdW1pbmc=", "T2zDoSEgaXN0byDDqSB1bSB0ZXN0ZQ==", "5L2g5aW977yM6L+Z5piv5LiA5Liq5rWL6K+V"];
        for (i, bytes) in input.iter().enumerate() {
            let vec = base64_encode_bytes(bytes);
            let res_str = str::from_utf8(&vec);
            assert_eq!(output[i], res_str.unwrap());
        }
    }

    #[test]
    fn when_base64_encode_should_return_success() {
        let res = base64_encode("free Command to Display the Amount of Physical and Swap Memory".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "ZnJlZSBDb21tYW5kIHRvIERpc3BsYXkgdGhlIEFtb3VudCBvZiBQaHlzaWNhbCBhbmQgU3dhcCBNZW1vcnk=");
    }

    #[test]
    fn when_decode_quartet_should_return_right_trio() {
        let input: [u8; 4] = [19, 22, 5, 46];
        let output = decode_quartet(&input);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], 77);
        assert_eq!(output[1], 97);
        assert_eq!(output[2], 110);
    }

    #[test]
    fn when_decode_trio_should_decode() {
        let raw_input: [u8; 4] = ['T' as u8, 'W' as u8, 'E' as u8, '=' as u8];
        let converted = convert_encoded_bytes(&raw_input);
        let bytes = converted.as_slice();
        assert_eq!(19, bytes[0]);
        assert_eq!(22, bytes[1]);
        assert_eq!(4, bytes[2]);
        let decoded = decode_incomplete(bytes);
        assert!(decoded.is_ok());
        let decoded_bytes = decoded.unwrap();
        assert_eq!(2, decoded_bytes.len());
        assert_eq!(77, decoded_bytes[0]);
        assert_eq!(97, decoded_bytes[1]);
    }

    #[test]
    fn when_decode_calc_byte_size_should_give_right_size() {
        fn perform_test(expected: usize, str: &str) {
            assert_eq!(expected, decode_calc_byte_size(str.as_bytes()));
        }

        perform_test(1, "TQ==");
        perform_test(2, "TWE=");
        perform_test(3, "TWFu");
        perform_test(4, "Zm91cg=="); // four
        perform_test(5, "dGhyZWU="); // three
        perform_test(6, "dGhyZWVz"); // threes
    }

    #[test]
    fn when_base64_decode_bytes_should_give_right_results() {
        check_decode("TWFu", "Man");
        check_decode("TWE=", "Ma");
        check_decode("TQ==", "M");
        check_decode("Zm91cg==", "four");
        check_decode("dGhyZWU=", "three");
        check_decode("dGhyZWVz", "threes");
    }

    fn check_decode(input: &str, expected: &str) {
        let res = base64_decode_bytes(input.as_bytes());
        assert!(res.is_ok());
        let decoded = res.unwrap();
        assert_eq!(expected.len(), decoded.len());
        assert_eq!(expected, str::from_utf8(&decoded).unwrap());
    }

    #[test]
    fn when_base64_decode_should_decode() {
        let data = "VGhpcyBpcyBncmVhdCBzdHVmZg=="
            .replace("+", "-").replace("/", "_");
        println!("{}", data);
        let res = base64_decode(data.to_string());
        assert!(res.is_ok());
        assert_eq!("This is great stuff", res.unwrap())
    }

    #[test]
    fn when_base64_encode_should_base64_decode() {
        for s in vec!["This is a nice text.", "Este é um texto super interessante!",
                      "एक बहुत अच्छी रात और एक अच्छा कल", "一个非常美好的夜晚和明天美好的一天"] {
            encode_decode_test(s);
        }
    }

    fn encode_decode_test(str: &str) {
        let encode_res = base64_encode(str.to_string());
        assert!(encode_res.is_ok());
        let encoded = encode_res.unwrap();
        let decoded = base64_decode(encoded);
        assert!(decoded.is_ok());
        let final_str = decoded.unwrap();
        assert_eq!(str, final_str);
    }

    #[test]
    fn when_base64_encode_to_file_should_create_file() {
        let sample_image = PathBuf::from("resources/sample_image.png");
        let target_image = PathBuf::from("sample_image_base64.txt");
        let res = base64_encode_to_file(sample_image, target_image);
        assert!(res.is_ok());
        let target_image_final = PathBuf::from("sample_image_base64.png");
        base64_decode_from_file(PathBuf::from("sample_image_base64.txt"),
                                target_image_final);
    }
}
