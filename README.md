# Image to OLED [![build badge](https://github.com/mdegraw/image-to-oled/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/mdegraw/image-to-oled/actions/workflows/rust.yml) [![docs.rs version](https://img.shields.io/docsrs/image-to-oled)](https://docs.rs/image-to-oled/latest/image-to-oled/)

Converts an [ImageBuffer](https://docs.rs/image/0.23.14/image/struct.ImageBuffer.html) to a 128x64 SSD1306 OLED byte array

## Example
```rust
let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
    ImageBuffer::from_vec(2, 2, vec![30; 12]).unwrap();
// get bytes array, specifying the brightness threshold 
// as a number between 0 and 255
let byte_array = to_oled_byte_array(&image_buffer, 30);
```