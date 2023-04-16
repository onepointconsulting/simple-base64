use lazy_static::lazy_static;

const BASE_64_ENCODING_URL: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const BASE_64_ENCODING: &'static str =     "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub const PADDING_CHAR: u8 = 61; // '=' character

pub const BASE_64_ENCODING_CHARS: &[u8] = BASE_64_ENCODING.as_bytes();
pub const BASE_64_ENCODING_CHARS_URL: &[u8] = BASE_64_ENCODING_URL.as_bytes();

lazy_static! {
    pub static ref CHARS_BASE_64_ENCODING: Vec<u8> = compute_reverse_encoding(BASE_64_ENCODING);
    pub static ref CHARS_BASE_64_ENCODING_URL: Vec<u8> = compute_reverse_encoding(BASE_64_ENCODING_URL);
}

fn compute_reverse_encoding(char_set: &str) -> Vec<u8> {
    let mut encoding: Vec<u8> = vec![0; 127];
    for (i, b) in char_set.as_bytes().to_vec().iter().enumerate() {
        encoding[*b as usize] = i as u8;
    }
    encoding[PADDING_CHAR as usize] = PADDING_CHAR;
    return encoding;
}