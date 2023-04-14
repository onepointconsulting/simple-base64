use lazy_static::lazy_static;

const BASE_64_ENCODING: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub const PADDING_CHAR: u8 = 61; // '=' character

pub const BASE_64_ENCODING_CHARS: &[u8] = BASE_64_ENCODING.as_bytes();

lazy_static! {
    pub static ref CHARS_BASE_64_ENCODING: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 62, 0, 0, 52, 53,
        54, 55, 56, 57, 58, 59, 60, 61, 0, 0,
        0, 65, 0, 0, 0, 0, 1, 2, 3, 4,
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 0, 0, 0, 0, 63, 0, 26, 27, 28,
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
        39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 0, 0, 0, 0, 0
    ];
}