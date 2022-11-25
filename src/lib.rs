use image::{ImageBuffer, Rgb};

pub type Frame = Vec<u8>;
pub type FrameBuffer = ImageBuffer<Rgb<u8>, Frame>;

/// Converts RGB pixels to a 128 x 64 SSD1306 OLED byte array
pub fn to_oled_byte_array(frame_buffer: &FrameBuffer, threshold: u8) -> Frame {
    let resized_img =
        image::imageops::resize(frame_buffer, 128, 64, image::imageops::FilterType::Nearest);

    resized_img
        .chunks(3)
        .fold(
            (0, 0, 7_i32, resized_img.len(), Vec::<u8>::new()),
            |(mut number, mut i, mut byte_index, pixels_len, mut oled_frame), rgb| {
                // Get the average of the RGB
                let avg: u8 = rgb.iter().sum::<u8>() / 3;

                if avg > threshold {
                    number += 2_u8.pow(byte_index as u32);
                }

                byte_index -= 1;

                // if this was the last pixel of a row or the last pixel of the
                // image, fill up the rest of our byte with zeros so it always contains 8 bits
                if (i != 0 && (((i / 3) + 1) % (128)) == 0) || (i == (pixels_len - 3)) {
                    byte_index = -1;
                }

                // When there are 8 bits push into Vec and reset counts
                if byte_index < 0 {
                    oled_frame.push(number);
                    number = 0;
                    byte_index = 7;
                }

                i += 3;

                (number, i, byte_index, pixels_len, oled_frame)
            },
        )
        .4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_byte_array_of_size_1024_for_any_size_frame_buffer() {
        for i in [1, 4, 8, 16, 200, 400, 1000, 10000] {
            let frame_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_vec(i, i, vec![0; (i * i * 3) as usize]).unwrap();
            let result = to_oled_byte_array(&frame_buffer, 20);

            assert_eq!(result.len(), 1024);
        }
    }

    #[test]
    fn it_uses_threshold_to_determine_number() {
        let frame_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_vec(2, 2, vec![30; 12]).unwrap();
        let avg_below_threshold_results = to_oled_byte_array(&frame_buffer, 30);

        assert_eq!(avg_below_threshold_results, vec![0; 1024]);
        assert_eq!(avg_below_threshold_results.len(), 1024);

        let avg_above_threshold_results = to_oled_byte_array(&frame_buffer, 20);

        assert_eq!(avg_above_threshold_results, vec![255; 1024]);
        assert_eq!(avg_above_threshold_results.len(), 1024);
    }
}
