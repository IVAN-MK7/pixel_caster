use image;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs, io, path::Path};

/// added because PixelsCollection was moved to a new module, "pub" in order to make it callable from this module pixels_string::PixelsCollection for backwards compatibility, to remove at version 2.0
pub use crate::PixelsCollection;
use crate::{add_limited, bgra_management::*, PixelValues, BGRA_INVISIBLE_PIXEL};

pub struct CharsCollectionCreator<'a> {
    pixels_collection: &'a PixelsCollection<u8>,
    start_x: usize,
    start_y: usize,
    range_x: usize,
    range_y: usize,
    min_px_space_between_chars: usize,
    chars_string: &'a str,
    space_char_width: u32,
    bgra_matcher: fn(u8, u8, u8, u8) -> bool,
}

impl<'a> CharsCollectionCreator<'a> {
    /// Tries to get the same amount of characters provided in chars_string from the whole `PixelsCollection`.
    /// Scans an area of the given range from the provided coordinates and gets the pixels that pass the bgra_matcher,
    /// with those pixels creates the most little rectange that still comprehends them.
    pub fn create(&self) -> Result<CharsCollection<u8>, String> {
        let Self {
            pixels_collection,
            mut start_x,
            start_y,
            mut range_x,
            range_y,
            min_px_space_between_chars,
            chars_string,
            space_char_width,
            bgra_matcher,
        } = *self;

        // edges, cardinal points of the range of pixels that pass the bgra_matcher (e.g. : bgra_matchers::visible = which werent transparent, where A > 0)
        let img_visible_range = get_cardinal_points_until_nonestreak_x(
            &pixels_collection.bytes,
            pixels_collection.height,
            start_x,
            start_y,
            range_x,
            range_y,
            range_x,
            bgra_matcher,
        );

        let original_range_x = range_x;

        let mut char_u8_vec = CharsCollection {
            chars: Vec::new(),
            path: "".to_string(),
            bgra: BGRA(0, 0, 0, 255),
        };

        for char in chars_string.chars() {
            // this char's cardinal points
            let values = get_cardinal_points_until_nonestreak_x(
                &pixels_collection.bytes,
                pixels_collection.height,
                start_x,
                start_y,
                range_x,
                range_y,
                min_px_space_between_chars,
                bgra_matcher,
            );

            // with this char's cardinal points creates the most little range that still comprehends them and using that
            // creates a Vec<u8> that will be populated only with this character's pixels. those not passing the matcher will have their color set to BGRA_INVISIBLE_PIXEL (B=G=R=A=0)
            let (pixels_captured, _) = pixel_grabber(
                &pixels_collection.bytes,
                pixels_collection.height,
                values.left_x,
                img_visible_range.top_y,
                values.right_x - values.left_x + 1,
                values.bottom_y - img_visible_range.top_y + 1,
                bgra_matcher,
            );

            // add this character to the collection
            char_u8_vec.chars.push(PixelsChar {
                char,
                char_name: CHARS.get_char_name_by_char(char).unwrap(),
                pixels: PixelsCollection::<u8>::create(
                    values.right_x - values.left_x + 1,
                    values.bottom_y - img_visible_range.top_y + 1,
                    pixels_captured,
                )
                .unwrap(),
            });

            // println!("printing: char:{}, {}", char, char_u8_vec.chars.last().unwrap().char);
            //image::save_buffer_with_format(format!("fonts/exports/chars/{}.png", char_u8_vec.chars.last().unwrap().char_name), &<u8>::swap_blue_with_red(&char_u8_vec.chars.last().unwrap().pixels.bytes), char_u8_vec.chars.last().unwrap().pixels.width as u32, char_u8_vec.chars.last().unwrap().pixels.height as u32, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();

            if char == chars_string.chars().last().unwrap() {
                break;
            }

            // adjust the range of the image in which search for the next character
            start_x = values.right_x + min_px_space_between_chars;
            if start_x > original_range_x {
                return Err("Could not retrieve all the characters".to_owned());
            }
            range_x = original_range_x - start_x;
        }

        char_u8_vec.chars.push(PixelsChar {
            char: ' ',
            char_name: CHARS.get_char_name_by_char(' ').unwrap(),
            pixels: PixelsCollection::<u8>::create(
                space_char_width as usize,
                char_u8_vec.chars[0].pixels.height,
                vec![0; space_char_width as usize * char_u8_vec.chars[0].pixels.height * 4],
            )
            .unwrap(),
        });

        Ok(char_u8_vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::{pixels_string::*, PixelsSendMode};

    const DISPLAY_RESULTS: bool = true;

    #[test]
    fn string_of_chars() {
        //let image_transparent_bkgrnd = PixelsCollection::from_png("fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
        let image_transparent_bkgrnd = PixelsCollection::<u8>::from_png(
            "fonts/exports/transparent_green_40px_chars_sample__transparent_background.png",
        )
        .unwrap();
        //let image_transparent_bkgrnd = PixelsCollection::from_png("media/chars_sample_40px_blue_whitebackground.png").unwrap();
        // send_bytes(&image_white_bkgrnd.bytes, &(image_white_bkgrnd.width as i32), &(image_white_bkgrnd.height as i32), &0, &0, 255);

        let buffer = PixelsCollection::white_background_to_transparency_gradient(
            &image_transparent_bkgrnd.bytes,
        );
        let min_px_space_btwn_chars = 10;
        let chars_string = r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#;
        // a b c d e f g h i j k l m n o p q r s t u v w x y z , . ? ! 0 1 2 3 4 5 6 7 8 9 0 - + / * \ _ @ # ( ) [ ] { } ; : " £ $ % & = ' ^
        let space_char_width = 10;

        let height = image_transparent_bkgrnd.height;
        let mut start_x = 0;
        let start_y = 0;
        let mut range_x = image_transparent_bkgrnd.width;
        let range_y = image_transparent_bkgrnd.height;

        // range the extreme pixels which werent transparent (where A > 0)
        let img_visible_range = get_cardinal_points_until_nonestreak_x(
            &buffer,
            height,
            start_x,
            start_y,
            range_x,
            range_y,
            range_x,
            |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
        );

        let original_range_x = range_x;

        if DISPLAY_RESULTS {
            let buffer_alpha_not_zero = buffer.clone();
            //crate::bgra_management::u8_bgra_pos_not_zero_set_pos(&mut buffer_alpha_not_zero, 3, 255,0,0,255);
            crate::Screen::update_area_custom(
                &buffer_alpha_not_zero,
                0,
                0,
                original_range_x as u32,
                range_y as u32,
                PixelsSendMode::AlphaEnabled,
            );
            //export Vec<u8> bytes into .png with image formatting
            image::save_buffer_with_format(
                format!("{}{}", "fonts/exports/", "testing_result_export.png"),
                &<u8>::swap_blue_with_red(&buffer_alpha_not_zero),
                range_x as u32,
                range_y as u32,
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )
            .unwrap();
        }

        let mut char_u8_vec: CharsCollection<u8> = CharsCollection {
            chars: Vec::new(),
            path: "".to_string(),
            bgra: BGRA(0, 0, 0, 255),
        };

        let mut bytes_chars_poles = buffer.clone();

        for char in chars_string.chars() {
            let values = get_cardinal_points_until_nonestreak_x(
                &buffer,
                height,
                start_x,
                start_y,
                range_x,
                range_y,
                min_px_space_btwn_chars,
                |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
            );

            // +1 because start and end values are included in the area, therefore if an area's first pixel is at 0 and it's last at 9 its range is 10, range is 9-0+1. Another e.g.: x starts at 10, ends at 40 : area = 31; 40 - 10 + 1
            let (pixels_captured, char_values) = pixel_grabber(
                &buffer,
                height,
                values.left_x,
                img_visible_range.top_y,
                values.right_x - values.left_x + 1,
                values.bottom_y - img_visible_range.top_y + 1,
                |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
            );

            if DISPLAY_RESULTS {
                let vec_pos_char = vec![
                    (values.left_x, img_visible_range.top_y),
                    (values.left_x, img_visible_range.bottom_y),
                    (values.right_x, img_visible_range.top_y),
                    (values.right_x, img_visible_range.bottom_y),
                ];
                bytes_chars_poles.set_positions_bgra(height, &vec_pos_char, 0, 255, 0, 255);
                let vec_pos_char_strict = vec![
                    (values.left_x, values.top_y),
                    (values.left_x, values.bottom_y),
                    (values.right_x, values.top_y),
                    (values.right_x, values.bottom_y),
                ];
                bytes_chars_poles.set_positions_bgra(
                    height,
                    &vec_pos_char_strict,
                    170,
                    255,
                    170,
                    255,
                );
                let vec_addresses_char = vec![
                    values.top_y_index,
                    values.left_x_index,
                    values.right_x_index,
                    values.bottom_y_index,
                ];
                bytes_chars_poles.set_addresses_bgra(&vec_addresses_char, 0, 0, 255, 255);
                crate::Screen::update_area_custom(
                    &bytes_chars_poles,
                    0,
                    (range_y + 10) as i32,
                    original_range_x as u32,
                    range_y as u32,
                    PixelsSendMode::AlphaEnabled,
                );

                // greys (B == G == R) too close to white (255) greater than a threshold will be set transparent (Alpha = 0), the others will be set to 0 (black), unless whites (where B || G || R == 255)
                // u8_grey_scale_into_black(&mut pixels_captured, 149);
                crate::Screen::update_area_custom(
                    &pixels_captured,
                    0,
                    ((range_y + 10) * 2) as i32,
                    (values.right_x - values.left_x) as u32 + 1,
                    (values.bottom_y - img_visible_range.top_y) as u32 + 1,
                    PixelsSendMode::AlphaEnabled,
                );

                let mut pixels_captured_clone = pixels_captured.clone();
                let vec_pos_char_relative = vec![
                    (0, 0),
                    (0, values.bottom_y - img_visible_range.top_y),
                    (char_values.right_x, 0),
                    (
                        char_values.right_x,
                        values.bottom_y - img_visible_range.top_y,
                    ),
                ];
                pixels_captured_clone.set_positions_bgra(
                    (values.bottom_y - img_visible_range.top_y) + 1,
                    &vec_pos_char_relative,
                    170,
                    255,
                    170,
                    255,
                );
                let vec_addresses_char_relative = vec![
                    char_values.top_y_index,
                    char_values.left_x_index,
                    char_values.right_x_index,
                    char_values.bottom_y_index,
                ];
                pixels_captured_clone.set_addresses_bgra(
                    &vec_addresses_char_relative,
                    0,
                    0,
                    255,
                    255,
                );
                crate::Screen::update_area_custom(
                    &pixels_captured_clone,
                    0,
                    ((range_y + 10) * 3) as i32,
                    (values.right_x - values.left_x) as u32 + 1,
                    (values.bottom_y - img_visible_range.top_y) as u32 + 1,
                    PixelsSendMode::AlphaEnabled,
                );
            }

            char_u8_vec.chars.push(PixelsChar {
                char,
                char_name: String::from(char),
                pixels: PixelsCollection::<u8>::create(
                    values.right_x - values.left_x + 1,
                    values.bottom_y - img_visible_range.top_y + 1,
                    pixels_captured,
                )
                .unwrap(),
            });

            if char == chars_string.chars().last().unwrap() {
                break;
            }

            start_x = values.right_x + min_px_space_btwn_chars;
            if start_x > original_range_x {
                break;
            }
            range_x = original_range_x - start_x;
        }

        if DISPLAY_RESULTS {
            let vec_pos_string = vec![
                (img_visible_range.left_x, img_visible_range.top_y),
                (img_visible_range.left_x, img_visible_range.bottom_y),
                (img_visible_range.right_x, img_visible_range.top_y),
                (img_visible_range.right_x, img_visible_range.bottom_y),
            ];
            bytes_chars_poles.set_positions_bgra(height, &vec_pos_string, 255, 255, 0, 255);
            let vec_addresses_string = vec![
                img_visible_range.top_y_index,
                img_visible_range.left_x_index,
                img_visible_range.right_x_index,
                img_visible_range.bottom_y_index,
            ];
            bytes_chars_poles.set_addresses_bgra(&vec_addresses_string, 255, 0, 255, 255);
            crate::Screen::update_area_custom(
                &bytes_chars_poles,
                0,
                ((range_y + 10) * 4) as i32,
                original_range_x as u32,
                range_y as u32,
                PixelsSendMode::AlphaEnabled,
            );
        }

        assert_eq!(
            char_u8_vec.chars.len(),
            chars_string.chars().count(),
            "Could not retrieve all the characters ({}/{} retrieved)",
            char_u8_vec.chars.len(),
            chars_string.chars().count()
        );

        char_u8_vec.chars.push(PixelsChar {
            char: ' ',
            char_name: String::from("space"),
            pixels: PixelsCollection::<u8>::create(
                space_char_width,
                char_u8_vec.chars[0].pixels.height,
                vec![0; space_char_width * char_u8_vec.chars[0].pixels.height * 4],
            )
            .unwrap(),
        });

        // crate::send_bytes(&char_u8_vec.chars[0].bgra_bytes, &(char_u8_vec.chars[0].width as i32), &(char_u8_vec.chars[0].height as i32), &10, &10, 255);

        let strings_from_string_png =
            Vec::from([char_u8_vec.create_pixels_string("testing generated_text!^", 3)]);
        crate::Screen::update_area_custom(
            &strings_from_string_png[0].pixels.bytes,
            0,
            ((range_y + 10) * 5) as i32,
            strings_from_string_png[0].pixels.width as u32,
            strings_from_string_png[0].pixels.height as u32,
            PixelsSendMode::AlphaEnabled,
        );
    }

    // TO DO: remove this fn, it was just for temp purpose, prints a gold pixel on the top left and top right side of each char at the height of the heighest char starting from each char's bottom

    #[test]
    fn string_of_chars_with_highest_char_sides() {
        //let image_transparent_bkgrnd = PixelsCollection::from_png("fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
        let image_transparent_bkgrnd = PixelsCollection::<u8>::from_png(
            "fonts/exports/transparent_green_40px_chars_sample__transparent_background.png",
        )
        .unwrap();
        //let image_transparent_bkgrnd = PixelsCollection::from_png("media/chars_sample_40px_blue_whitebackground.png").unwrap();
        // send_bytes(&image_white_bkgrnd.bytes, &(image_white_bkgrnd.width as i32), &(image_white_bkgrnd.height as i32), &0, &0, 255);

        let buffer = image_transparent_bkgrnd.bytes.clone();
        let min_px_space_btwn_chars = 8;
        let chars_string = r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#;
        // a b c d e f g h i j k l m n o p q r s t u v w x y z , . ? ! 0 1 2 3 4 5 6 7 8 9 0 - + / * \ _ @ # ( ) [ ] { } ; : " £ $ % & = ' ^

        let height = image_transparent_bkgrnd.height;
        let mut start_x = 0;
        let start_y = 0;
        let mut range_x = image_transparent_bkgrnd.width;
        let range_y = image_transparent_bkgrnd.height;

        // range the extreme pixels which werent transparent (where A > 0)
        let img_visible_range = get_cardinal_points_until_nonestreak_x(
            &buffer,
            height,
            start_x,
            start_y,
            range_x,
            range_y,
            range_x,
            |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
        );

        let original_range_x = range_x;

        let mut bytes_chars_poles = buffer.clone();

        let mut highest_height = 0;

        for char in chars_string.chars() {
            let values = get_cardinal_points_until_nonestreak_x(
                &buffer,
                height,
                start_x,
                start_y,
                range_x,
                range_y,
                min_px_space_btwn_chars,
                |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
            );

            // +1 because start and end values are included in the area, therefore if an area's first pixel is at 0 and it's last at 9 its range is 10, range is 9-0+1. Another e.g.: x starts at 10, ends at 40 : area = 31; 40 - 10 + 1
            let (_, char_values) = pixel_grabber(
                &buffer,
                height,
                values.left_x,
                img_visible_range.top_y,
                values.right_x - values.left_x + 1,
                values.bottom_y - img_visible_range.top_y + 1,
                |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
            );

            if DISPLAY_RESULTS {
                let vec_pos_char = vec![
                    (values.left_x, img_visible_range.top_y),
                    (values.left_x, img_visible_range.bottom_y),
                    (values.right_x, img_visible_range.top_y),
                    (values.right_x, img_visible_range.bottom_y),
                ];
                bytes_chars_poles.set_positions_bgra(height, &vec_pos_char, 0, 255, 0, 255);
                let vec_pos_char_strict = vec![
                    (values.left_x, values.top_y),
                    (values.left_x, values.bottom_y),
                    (values.right_x, values.top_y),
                    (values.right_x, values.bottom_y),
                ];
                bytes_chars_poles.set_positions_bgra(
                    height,
                    &vec_pos_char_strict,
                    170,
                    255,
                    170,
                    255,
                );
                let vec_addresses_char = vec![
                    values.top_y_index,
                    values.left_x_index,
                    values.right_x_index,
                    values.bottom_y_index,
                ];
                bytes_chars_poles.set_addresses_bgra(&vec_addresses_char, 0, 0, 255, 255);

                highest_height =
                    std::cmp::max(highest_height, char_values.bottom_y - char_values.top_y);
            }

            if char == chars_string.chars().last().unwrap() {
                break;
            }

            start_x = values.right_x + min_px_space_btwn_chars;
            if start_x > original_range_x {
                break;
            }
            range_x = original_range_x - start_x;
        }

        let mut start_x = 0;
        let mut range_x = image_transparent_bkgrnd.width;

        for char in chars_string.chars() {
            let values = get_cardinal_points_until_nonestreak_x(
                &buffer,
                height,
                start_x,
                start_y,
                range_x,
                range_y,
                min_px_space_btwn_chars,
                |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
            );

            if DISPLAY_RESULTS {
                let heighest_char_top_from_this_char_bottom_side_marks = vec![
                    (values.left_x - 2, values.bottom_y - highest_height),
                    (values.right_x + 2, values.bottom_y - highest_height),
                ];
                bytes_chars_poles.set_positions_bgra(
                    height,
                    &heighest_char_top_from_this_char_bottom_side_marks,
                    161,
                    248,
                    255,
                    255,
                );

                crate::Screen::update_area_custom(
                    &bytes_chars_poles,
                    0,
                    (range_y + 10) as i32,
                    original_range_x as u32,
                    range_y as u32,
                    PixelsSendMode::AlphaEnabled,
                );
            }
            if char == chars_string.chars().last().unwrap() {
                break;
            }

            start_x = values.right_x + min_px_space_btwn_chars;
            if start_x > original_range_x {
                break;
            }
            range_x = original_range_x - start_x;
        }

        image::save_buffer_with_format(
            "fonts/exports/with_poles.png",
            &<u8>::swap_blue_with_red(&bytes_chars_poles),
            image_transparent_bkgrnd.width as u32,
            image_transparent_bkgrnd.height as u32,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }

    #[test]
    fn hashmap_test() {
        assert_eq!(
            find_key_for_value(&CHARS, 'a'),
            Some("LATIN SMALL LETTER A".to_string())
        );
        assert_eq!(
            find_key_for_value(&CHARS, 'A'),
            Some("LATIN CAPITAL LETTER A".to_string())
        );

        //CHARS.insert("zero".to_string(), '0');
        //CHARS.insert("A".to_string(), 'A');
        assert_eq!(CHARS.get_char_by_char_name_with_default("DIGIT ZERO"), '0');
        assert_eq!(CHARS.get_char_by_char_name_with_default("banana"), '█');
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct CardinalPoints {
    pub top_y: usize,
    pub top_y_index: usize,
    pub right_x: usize,
    pub right_x_index: usize,
    pub left_x: usize,
    pub left_x_index: usize,
    pub bottom_y: usize,
    pub bottom_y_index: usize,
}

impl CardinalPoints {
    /// If the provided point is lesser than the leftest cardinal point, it's value and index will be set to the leftest point's coordinates.
    /// If the provided point is greater than the rightest cardinal point, it's value and index will be set to the rightest point's coordinates.
    pub fn update_if_expands_horizontal(&mut self, point: usize, index: usize) {
        if point < self.left_x {
            self.left_x = point;
            self.left_x_index = index;
        }
        if point > self.right_x {
            self.right_x = point;
            self.right_x_index = index;
        }
    }

    /// If the provided point is lesser than the top cardinal point, it's value and index will be set to the top point's coordinates.
    /// If the provided point is greater than the bottom cardinal point, it's value and index will be set to the bottom point's coordinates.
    pub fn update_if_expands_vertical(&mut self, point: usize, index: usize) {
        if point < self.top_y {
            self.top_y = point;
            self.top_y_index = index;
        }
        if point > self.bottom_y {
            self.bottom_y = point;
            self.bottom_y_index = index;
        }
    }

    /// If the provided CardinalPoints' have some outermost values than the self's, these will be set as the new values.
    pub fn update_if_expands_from_cardinal_points(&mut self, c_p: &CardinalPoints) {
        self.update_if_expands_horizontal(c_p.left_x, c_p.left_x_index);
        self.update_if_expands_horizontal(c_p.right_x, c_p.right_x_index);
        self.update_if_expands_vertical(c_p.top_y, c_p.top_y_index);
        self.update_if_expands_vertical(c_p.bottom_y, c_p.bottom_y_index);
    }

    pub fn width(&self) -> Result<usize, String> {
        if self.right_x < self.left_x {
            return Err("No fully opaque pixels found".to_string());
        }
        Ok(self.right_x - self.left_x + 1)
    }

    pub fn height(&self) -> Result<usize, String> {
        if self.bottom_y < self.top_y {
            return Err("No fully opaque pixels found".to_string());
        }
        Ok(self.bottom_y - self.top_y + 1)
    }
}

impl Default for CardinalPoints {
    fn default() -> Self {
        Self {
            top_y: usize::MAX,
            top_y_index: 0,
            right_x: 0,
            right_x_index: 0,
            left_x: usize::MAX,
            left_x_index: 0,
            bottom_y: 0,
            bottom_y_index: 0,
        }
    }
}

impl std::fmt::Debug for CardinalPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardinalPoints")
            .field("top_y", &self.top_y)
            .field("top_y_index", &self.top_y_index)
            .field("right_x", &self.right_x)
            .field("right_x_index", &self.right_x_index)
            .field("left_x", &self.left_x)
            .field("left_x_index", &self.left_x_index)
            .field("bottom_y", &self.bottom_y)
            .field("bottom_y_index", &self.bottom_y_index)
            .finish()
    }
}

/// returns the cardinal points of given range of which pixels match a condition
pub fn get_cardinal_points_until_nonestreak_x(
    buffer: &[u8],
    height: usize,
    start_x: usize,
    start_y: usize,
    range_x: usize,
    range_y: usize,
    none_streak_x: usize,
    bgra_matcher: fn(u8, u8, u8, u8) -> bool,
) -> CardinalPoints {
    // how many buffer units there are in a horizontal, 1 pixel high, line across the screen
    let stride = buffer.len() / height;

    let mut values = CardinalPoints {
        top_y: start_y + range_y,
        top_y_index: 0,
        right_x: 0,
        right_x_index: 0,
        left_x: start_x + range_x,
        left_x_index: 0,
        bottom_y: 0,
        bottom_y_index: 0,
    };

    for x in start_x..start_x + range_x {
        for y in start_y..start_y + range_y {
            let i = stride * y + 4 * x;
            if bgra_matcher(buffer[i], buffer[i + 1], buffer[i + 2], buffer[i + 3]) {
                if y < values.top_y {
                    values.top_y = y;
                    values.top_y_index = i;
                }
                if x < values.left_x {
                    values.left_x = x;
                    values.left_x_index = i;
                }
                if x > values.right_x {
                    values.right_x = x;
                    values.right_x_index = i;
                }
                if y > values.bottom_y {
                    values.bottom_y = y;
                    values.bottom_y_index = i;
                }
            }
        }
        if values.right_x > start_x && x >= values.right_x + none_streak_x {
            break;
        }
    }
    values
}

/// From a starting pixel scans an area of the given range and populates a new Vec<u8> with the given range with pixels that pass the bgra_matcher.
/// Returns the Vec<u8> and the cardinal points of the most outer pixels that passed the bgra_matcher
pub fn pixel_grabber(
    buffer: &[u8],
    height: usize,
    start_x: usize,
    start_y: usize,
    range_x: usize,
    range_y: usize,
    bgra_matcher: fn(u8, u8, u8, u8) -> bool,
) -> (Vec<u8>, CardinalPoints) {
    // how many buffer units there are in a horizontal, 1 pixel high, line across the screen
    let stride = buffer.len() / height;

    // will contain all the pixels which werent transparent (where A > 0)
    let mut pixels_captured: Vec<u8> = Vec::with_capacity(range_x * range_y * 4);
    let mut values = CardinalPoints {
        top_y: start_y + range_y,
        top_y_index: 0,
        right_x: 0,
        right_x_index: 0,
        left_x: start_x + range_x,
        left_x_index: 0,
        bottom_y: 0,
        bottom_y_index: 0,
    };
    let mut rx = 0;
    let rstride = (range_x * range_y * 4) / range_y;
    for (ry, y) in (start_y..start_y + range_y).enumerate() {
        for x in start_x..start_x + range_x {
            let i = stride * y + 4 * x;
            let j = rstride * ry + 4 * rx;
            if bgra_matcher(buffer[i], buffer[i + 1], buffer[i + 2], buffer[i + 3]) {
                // is in BGRA
                pixels_captured.extend_from_slice(&[
                    buffer[i],
                    buffer[i + 1],
                    buffer[i + 2],
                    buffer[i + 3],
                ]);

                if ry < values.top_y {
                    values.top_y = ry;
                    values.top_y_index = j;
                }
                if rx < values.left_x {
                    values.left_x = rx;
                    values.left_x_index = j;
                }
                if rx > values.right_x {
                    values.right_x = rx;
                    values.right_x_index = j;
                }
                if ry > values.bottom_y {
                    values.bottom_y = ry;
                    values.bottom_y_index = j;
                }
            } else {
                pixels_captured.extend_from_slice(&[
                    BGRA_INVISIBLE_PIXEL.0,
                    BGRA_INVISIBLE_PIXEL.1,
                    BGRA_INVISIBLE_PIXEL.2,
                    BGRA_INVISIBLE_PIXEL.3,
                ]);
            }
            rx += 1;
        }
        rx = 0;
    }
    (pixels_captured, values)
}


/// Width and height restricted to the fully opaque area of the provided image.
pub fn get_fully_opaque_area_range(image: &PixelsCollection<u8>) -> Result<(usize, usize), String> {
    let values = get_cardinal_points_until_nonestreak_x(&image.bytes, image.height, 0, 0, image.width, image.height, image.width, |_,_,_,a| a == 255);
    Ok((values.width()?, values.height()?))
}


/// From a starting pixel scans an area of the given range and populates a new Vec<u8> with the given range with pixels.
pub fn pixels_from_area(buffer: &[u8], height: usize, start_x: usize, start_y: usize, range_x: usize, range_y: usize) -> Vec<u8> {
    
    // how many buffer units there are in a horizontal, 1 pixel high, line across the screen
    let stride = buffer.len() / height;

    let mut pixels_captured = Vec::with_capacity(range_x * range_y * 4);
    for y in start_y..start_y+range_y {
        for x in start_x..start_x+range_x {
            let i = stride * y + 4 * x;
            pixels_captured.extend_from_slice(&[ buffer[i], buffer[i+1], buffer[i+2], buffer[i+3] ]);
        }
    }
    pixels_captured
}

/// Additional implementations that enables .png importing and CharsCollection creation
impl PixelsCollection<u8> {
    /// Creates a new instance from a .png (resulting color bytes will be BGRA ordered)
    pub fn from_png(png_path: &str) -> Result<PixelsCollection<u8>, String> {
        // get Vec<u8> from .png and load it to a .png format, png works in RGBA, to make it usable it will be converted into BGRA
        match crate::pixels_string::png_into_pixels_collection(png_path) {
            Ok(mut pixel_coll) => {
                pixel_coll.switch_bytes(0, 2);
                Ok(pixel_coll)
            }
            Err(err) => Err(err),
        }
    }
    /// Tries to get the same amount of characters provided in chars_string from the whole PixelsCollection
    pub fn try_create_char_collection(
        &self,
        min_px_space_between_chars: usize,
        chars_string: &str,
        space_char_width: u32,
        bgra_matcher: fn(u8, u8, u8, u8) -> bool,
    ) -> Result<CharsCollection<u8>, String> {
        let start_x = 0;
        let start_y = 0;
        let range_x = self.width;
        let range_y = self.height;

        let char_coll_creator = CharsCollectionCreator {
            pixels_collection: self,
            start_x,
            start_y,
            range_x,
            range_y,
            min_px_space_between_chars,
            chars_string,
            space_char_width,
            bgra_matcher,
        };
        match char_coll_creator.create() {
            Ok(mut coll) => {
                coll.bgra = image_lowest_visible_bgr(&self.bytes);
                Ok(coll)
            }
            Err(err) => Err(err),
        }
    }
}
impl PixelsCollection<u32> {
    /// Creates a new instance from a .png (resulting color bytes will be BGRA ordered)
    pub fn from_png(png_path: &str) -> Result<PixelsCollection<u32>, String> {
        // get Vec<u8> from .png and load it to a .png format, png works in RGBA, to make it usable it will be converted into BGRA
        match crate::pixels_string::png_into_pixels_collection(png_path) {
            Ok(mut pixel_coll) => {
                pixel_coll.switch_bytes(0, 2);
                Ok(PixelsCollection::<u32>::create(
                    pixel_coll.width,
                    pixel_coll.height,
                    <u8>::u8_u32_casting(&pixel_coll.bytes),
                )?)
            }
            Err(err) => Err(err),
        }
    }
}

/// Gets a Vec<u8> in RGBA values from a .png
pub fn png_into_pixels_collection(png_path: &str) -> Result<PixelsCollection<u8>, String> {
    match image::open(png_path) {
        Ok(img) => Ok(PixelsCollection::<u8>::create(
            img.width() as usize,
            img.height() as usize,
            img.into_rgba8().to_vec(),
        )?),
        Err(err) => Err(err.to_string()),
    }
    /*match std::fs::read(png_path) {
        Ok(bytes) => {
            match image::load_from_memory_with_format(&bytes, image::ImageFormat::Png) {
                Ok(img) => {
                    // If .png was made with GIMP, ensure that when exported, the voice "Save color profile" is unchecked
                    // println!("imported png : w{} y{}", img.width(), img.height());
                    // pixel_caster::send_bytes(&mut img.clone().as_bytes().to_vec(), &(img.width() as i32), &(img.height() as i32), &50, &800);
                    // working with rgba16 would be pointless, since the winapi works only with rgba8 (0-255 values)
                    return Ok( PixelsCollection::<u8>::create(img.width() as usize, img.height() as usize, img.into_rgba8().to_vec() )?);
                    //return PixelsCollection::create(img.width() as usize, img.height() as usize, img.into_bytes() );
                }
                Err(_) => {
                    return Err("input is not png".to_string());
                }
            };
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }*/
}

pub fn create_dir_recursive(dir_full_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir_full_path)?;
    Ok(())
}

/// Character name, the char it refers to and its PixelsCollection
#[derive(Clone)]
pub struct PixelsChar<T: PixelValues<T>> {
    pub char: char,
    pub char_name: String,
    pub pixels: PixelsCollection<T>,
}
impl PixelsChar<u8> {
    /// Creates a new instance that will represent the given char
    pub fn create(
        char: char,
        char_name: &str,
        width: usize,
        height: usize,
        bytes: Vec<u8>,
    ) -> Result<PixelsChar<u8>, String> {
        Ok(PixelsChar {
            char,
            char_name: char_name.to_string(),
            pixels: PixelsCollection::<u8>::create(width, height, bytes)?,
        })
    }

    /// Creates a new instance from a .png (resulting color bytes will be BGRA ordered)
    pub fn from_png(png_path: &str, char: char, char_name: &str) -> Result<PixelsChar<u8>, String> {
        // get Vec<u8> from .png and load it to a .png format, png works in RGBA, to make it usable it will be converted into BGRA
        match crate::pixels_string::png_into_pixels_collection(png_path) {
            Ok(mut bytes) => {
                bytes.switch_bytes(0, 2);
                Ok(PixelsChar {
                    char,
                    char_name: char_name.to_string(),
                    pixels: bytes,
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn switch_bytes(&mut self, i1: usize, i2: usize) {
        <u8>::switch_bytes(&mut self.pixels.bytes, i1, i2);
    }
}

#[derive(Clone, Copy)]
pub struct BGRA<T: Copy + Clone>(pub T, pub T, pub T, pub T);
impl<T: Copy + Clone> BGRA<T> {
    pub fn to_vec(&self) -> Vec<T> {
        vec![self.0, self.1, self.2, self.3]
    }
}

/// Collection of PixelsChar
#[derive(Clone)]
pub struct CharsCollection<T: PixelValues<T> + Copy + Clone> {
    pub chars: Vec<PixelsChar<T>>,
    pub path: String,
    pub bgra: BGRA<T>,
}

impl CharsCollection<u8> {
    /*pub fn from_pixelsvec (pv: &PixelsVec, start_x :usize, start_y :usize, mut range_x :usize, mut range_y :usize, min_px_space_btwn_chars :usize, chars_string :&str, space_char_width :u32) -> Self {
        char_collection_from_image_with_transparency(&pv.bytes, pv.height, start_x, start_y, range_x, range_y, min_px_space_btwn_chars, chars_string, space_char_width)
    }*/
    /// Creates a new collection from a folder containing the chars in .png file format.
    /// The filenames that do not match those inside the CHARS hashmap will still be added, but will represent the default char '█'
    pub fn from_pngs_folder(dir: &str) -> io::Result<CharsCollection<u8>> {
        let mut char_u8_vec = CharsCollection {
            chars: Vec::new(),
            path: dir.to_string(),
            bgra: BGRA(0, 0, 0, 255),
        };

        for entry in fs::read_dir(Path::new(dir))? {
            let entry = entry?;
            let path = entry.path();
            // path is directory && get extension from filename == "png"
            if !path.is_dir() && path.extension().and_then(OsStr::to_str).unwrap() == "png" {
                let fname_without_extension =
                    String::from(path.file_stem().unwrap().to_str().unwrap());
                let fpath = path.into_os_string().into_string().unwrap();
                //println!("{:?}, {:?}, {:?}",fname_without_extension, fname, fpath);

                // get Vec<u8> from .png and load it to a .png format, png works in RGBA, to make it usable it will be converted into BGRA
                match PixelsChar::from_png(
                    &fpath,
                    CHARS.get_char_by_char_name_with_default(&fname_without_extension),
                    &fname_without_extension,
                ) {
                    Ok(mut pixels_char) => {
                        pixels_char.switch_bytes(0, 2);
                        char_u8_vec.chars.push(pixels_char);
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        Ok(char_u8_vec)
    }
    /// Exports the collection's chars' Vec<u8> color bytes into the given folder path in .png file format (BGRA will become RGBA)
    pub fn export_as_pngs(&self, folder_path: &str) -> std::io::Result<()> {
        Self::export(folder_path, self)
    }
    /// Exports all the Chars into the CharsCollection's path except those whose char value is the one provided
    /// (e.g.: set to '█' in order to not to export the chars which char_name was not present in CHARS at the moment of importing it froma folder of .png chars)
    pub fn export_as_pngs_overwrite_except_char(
        &mut self,
        folder_path: &str,
        c_to_exclude: char,
    ) -> std::io::Result<()> {
        let mut cc_except = CharsCollection {
            chars: self.chars.clone(),
            path: self.path.to_string(),
            bgra: self.bgra,
        };
        cc_except.chars.retain(|x| x.char != c_to_exclude);
        Self::export(
            &format!(
                "{}{}",
                folder_path.trim_end_matches('\\'),
                "\\mapped_in_CHARS\\"
            ),
            &cc_except,
        )
    }
    /// Exports the provided collection's chars' Vec<u8> color bytes into the given folder path in .png file format (BGRA will become RGBA), creates the path if the provided one does not exist
    fn export(png_path: &str, coll: &CharsCollection<u8>) -> std::io::Result<()> {
        match create_dir_recursive(png_path) {
            Ok(()) => {}
            Err(err) => {
                return Err(err);
            }
        };
        for c in &coll.chars {
            image::save_buffer_with_format(
                format!(
                    "{}{}.png",
                    png_path.trim_end_matches('\\').to_owned() + "\\",
                    c.char_name
                ),
                &<u8>::swap_blue_with_red(&c.pixels.bytes),
                c.pixels.width as u32,
                c.pixels.height as u32,
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )
            .unwrap();
        }
        Ok(())
    }
}

impl CharsCollection<u8> {
    /// Every grey below a threshold set to black, else to invisible
    pub fn grey_scale_into_black(&mut self, grey_threshold: u8) {
        for v in &mut self.chars {
            PixelsCollection::grey_scale_into_black(&mut v.pixels.bytes, grey_threshold);
        }
    }
    /// If a BGRA combination is met, set it to a provided BGRA
    pub fn matching_color_change(
        &mut self,
        b: u8,
        g: u8,
        r: u8,
        a: u8,
        new_b: u8,
        new_g: u8,
        new_r: u8,
        new_a: u8,
    ) {
        for v in &mut self.chars {
            v.pixels.bytes.color_matcher_and_new_color(
                |v0: u8, v1: u8, v2: u8, v3: u8| -> bool {
                    v0 == b && v1 == g && v2 == r && v3 == a
                },
                new_b,
                new_g,
                new_r,
                new_a,
            );
        }
    }
    /// Sets BGRA for the invisible (/not displayed, Alpha = 0) pixels (which BGRA color matches BGRA_INVISIBLE_PIXEL)
    pub fn set_bgra_for_invisible(&mut self, b: u8, g: u8, r: u8, a: u8) {
        self.chars
            .iter_mut()
            .for_each(|c| c.pixels.set_bgra_for_invisible(b, g, r, a));
    }
    /// If a color's Alpha matches, set its BGR values
    pub fn alpha_match_set_bgr(&mut self, a: u8, b: u8, g: u8, r: u8) {
        self.chars.iter_mut().for_each(|x| {
            x.pixels.bytes.color_matcher_and_new_color(
                |_: u8, _: u8, _: u8, v3: u8| -> bool { v3 == a },
                b,
                g,
                r,
                a,
            )
        });
        self.bgra = BGRA(b, g, r, a);
    }

    /// Set the provided BGR
    pub fn set_bgr(&mut self, b: u8, g: u8, r: u8) {
        self.chars
            .iter_mut()
            .for_each(|x| x.pixels.bytes.set_bgr(b, g, r));
        self.bgra = BGRA(b, g, r, self.bgra.3);
    }

    /// Set the provided BGR
    pub fn set_bgr_from_array(&mut self, bgr: [u8; 3]) {
        let (b, g, r) = (bgr[0], bgr[1], bgr[2]);
        self.chars
            .iter_mut()
            .for_each(|x| x.pixels.bytes.set_bgr(b, g, r));
        self.bgra = BGRA(b, g, r, self.bgra.3);
    }
    
    pub fn create_pixels_string(&self, string: &str, char_spacing: isize) -> PixelsString {
        let mut vec: Vec<u8> = Vec::new();
        let mut vec_width = 0;
        // starting from a base value of 1, gets the tallest char's hight
        let tallest_char_height = self.chars.iter().fold(1, |a, b| a.max(b.pixels.height));
        // starting from a base value of 1, gets the widest char's width
        let widest_char_width = self.chars.iter().fold(1, |a, b| a.max(b.pixels.width));
        // add pixels to the resulting pixels string vector row by row, left to righ
        for string_y in 0..tallest_char_height {
            // char by char, when current char's row bytes end, the next character's row will start to be added
            for s in string.chars() {
                let char_found = self.chars.iter().find(|r| r.char == s);
                if let Some(char) = char_found {
                    let char_vec = &char.pixels.bytes;

                    // char width + char_spacing (char_spacing can be negative, which will remove columns starting from the last one on the right)
                    //let char_total_width = add_limited!(char.pixels.width as i32, char_spacing, 1);
                    let char_total_width = add_limited!(char.pixels.width as i32, char_spacing, 0);

                    // in case this char is not as high as the current row we won't have any more bytes to add
                    // so add the needed rows at the end to make it high as needed
                    if char.pixels.height <= string_y {
                        for _ in 0..char_total_width {
                            vec.extend_from_slice(&[0, 0, 0, 0]);
                        }
                        continue;
                    }
                    // start adding bytes to the resulting pixels string's current row
                    for w in 0..char_total_width {
                        // in case the char's current row's bytes have ended, add the necessary bytes to make up the remaining width (made by char_spacing)
                        if w >= char.pixels.width as i32 {
                            vec.extend_from_slice(&[0, 0, 0, 0]);
                        }
                        // add the char's current row's bytes to the resulting pixels string
                        else {
                            let i =
                                (char_vec.len() / char.pixels.height) * string_y + (w * 4) as usize;
                            vec.extend_from_slice(&[
                                char_vec[i],
                                char_vec[i + 1],
                                char_vec[i + 2],
                                char_vec[i + 3],
                            ])
                        }
                    }
                    // increments resulting pixels string's width with the current char's total width (occurrs once per each char)
                    //if string_y == 1 {
                    if string_y == 0 {
                        vec_width += char_total_width as usize;
                    }
                }
                // in case a char was not found, don't continue to next char, instead put widest_char_width wide matching bgra pixels
                else {
                    for _ in 0..add_limited!(
                        widest_char_width as i32,
                        char_spacing,
                        widest_char_width as i32
                    ) {
                        vec.extend_from_slice(&[self.bgra.0, self.bgra.1, self.bgra.2, 255]);
                    }
                    // increments resulting pixels string's width with the current char's total width (occurrs once per each char)
                    //if string_y == 1 {
                    if string_y == 0 {
                        vec_width += add_limited!(
                            widest_char_width as i32,
                            char_spacing,
                            widest_char_width as i32
                        ) as usize;
                    }
                    continue;
                }
            }
            // when the last character's current row bytes end, go back to the first character and start with the next row
        }
        PixelsString {
            bgra: self.bgra,
            pixels: PixelsCollection::<u8>::create(vec_width, tallest_char_height, vec).unwrap(),
        }
    }
}

/// A PixelsCollection obtained from the sum of some PixelsChar bytes, used to create a string chars to be exported or displayed
#[derive(Clone)]
pub struct PixelsString {
    pub bgra: BGRA<u8>,
    pub pixels: PixelsCollection<u8>,
}

/// Returns Some(the first Key that has the provided Value), otherwise None
fn find_key_for_value(
    map: &std::collections::HashMap<String, char>,
    value: char,
) -> Option<String> {
    map.iter().find_map(|(key, &val)| {
        if val == value {
            Some(key.to_owned())
        } else {
            None
        }
    })
}
pub trait CharsHashmap {
    fn get_char_by_char_name_with_default(&self, char_name: &str) -> char;
    fn get_char_name_by_char(&self, char: char) -> Option<String>;
}
impl CharsHashmap for std::collections::HashMap<String, char> {
    /// Returns the char corresponding to the char name, otherwise the char '█'
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    ///
    /// let mut chars = HashMap::new();
    ///
    /// chars.insert("DIGIT ZERO".to_string(), '0');
    /// chars.insert("LATIN CAPITAL LETTER A".to_string(), 'A');
    /// assert_eq!(chars.get_char_by_char_name_with_default("DIGIT ZERO"), '0');
    /// assert_eq!(chars.get_char_by_char_name_with_default("LATIN CAPITAL LETTER B"), '█');
    /// ```
    fn get_char_by_char_name_with_default(&self, char_name: &str) -> char {
        match self.get(char_name) {
            Some(x) => *x,
            None => '█',
        }
    }
    /// Returns Some(the first Key that has the provided Value), otherwise None
    fn get_char_name_by_char(&self, char: char) -> Option<String> {
        find_key_for_value(self, char)
    }
}

lazy_static! {
    /// Default hashmap with the character names and the char value each of them refers to
    pub static ref CHARS: std::collections::HashMap<String, char> = {
        let mut chars = std::collections::HashMap::new();

        // https://www.charset.org/utf-8
        // https://www.utf8-chartable.de/
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            chars.insert("LATIN CAPITAL LETTER ".to_string() + &c.to_string(), c);
            chars.insert("LATIN SMALL LETTER ".to_string() + &c.to_string(), c.to_lowercase().collect::<Vec<_>>()[0]);
        }

        chars.insert("DIGIT ZERO".to_string(), '0');
        chars.insert("DIGIT ONE".to_string(), '1');
        chars.insert("DIGIT TWO".to_string(), '2');
        chars.insert("DIGIT THREE".to_string(), '3');
        chars.insert("DIGIT FOUR".to_string(), '4');
        chars.insert("DIGIT FIVE".to_string(), '5');
        chars.insert("DIGIT SIX".to_string(), '6');
        chars.insert("DIGIT SEVEN".to_string(), '7');
        chars.insert("DIGIT EIGHT".to_string(), '8');
        chars.insert("DIGIT NINE".to_string(), '9');

        chars.insert("AMPERSAND".to_string(), '&');
        chars.insert("CIRCUMFLEX ACCENT".to_string(), '^');
        chars.insert("ASTERISK".to_string(), '*');
        chars.insert("REVERSE SOLIDUS".to_string(), '\\');
        chars.insert("VERTICAL LINE".to_string(), '|');
        chars.insert("LEFT CURLY BRACKET".to_string(), '{');
        chars.insert("RIGHT CURLY BRACKET".to_string(), '}');
        chars.insert("LEFT SQUARE BRACKET".to_string(), '[');
        chars.insert("RIGHT SQUARE BRACKET".to_string(), ']');
        chars.insert("COLON".to_string(), ':');
        chars.insert("COMMA".to_string(), ',');
        chars.insert("DEGREE SIGN".to_string(), '°');
        chars.insert("DIVISION SIGN".to_string(), '÷');
        chars.insert("EQUALS SIGN".to_string(), '=');
        chars.insert("PERCENT SIGN".to_string(), '%');
        chars.insert("EXCLAMATION MARK".to_string(), '!');
        chars.insert("GREATER-THAN SIGN".to_string(), '>');
        chars.insert("LESS-THAN SIGN".to_string(), '<');
        chars.insert("HYPHEN-MINUS".to_string(), '-');
        chars.insert("LEFT PARENTHESIS".to_string(), '(');
        chars.insert("RIGHT PARENTHESIS".to_string(), ')');
        chars.insert("AMPERSAND".to_string(), '&');
        chars.insert("FULL STOP".to_string(), '.');
        chars.insert("PLUS SIGN".to_string(), '+');
        chars.insert("QUESTION MARK".to_string(), '?');
        chars.insert("QUOTATION MARK".to_string(), '"');
        chars.insert("APOSTROPHE".to_string(), '\'');
        chars.insert("SEMICOLON".to_string(), ';');
        chars.insert("SOLIDUS".to_string(), '/');
        chars.insert("SPACE".to_string(), ' ');
        chars.insert("LOW LINE".to_string(), '_');
        chars.insert("COMMERCIAL AT".to_string(), '@');
        chars.insert("NUMBER SIGN".to_string(), '#');
        chars.insert("POUND SIGN".to_string(), '£');
        chars.insert("DOLLAR SIGN".to_string(), '$');
        chars.insert("EURO SIGN".to_string(), '€');


        chars
    };
}
