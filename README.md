# Simple BASE64 Encoder and Decoder

This project provides a Rust library with methods for encoding and decoding in Base 64.

It provides methods for encoding and decoding bytes, string and files.

Here is an example of its usage:

```rust
let sample_image = PathBuf::from("resources/sample_image.png");
let target_image = PathBuf::from("sample_image_base64.txt");
let res = base64_encode_to_file(sample_image, target_image);
assert!(res.is_ok());
let target_image_final = PathBuf::from("sample_image_base64.png");
base64_decode_from_file(PathBuf::from("sample_image_base64.txt"),
                        target_image_final);
```

