#![cfg(test)]

use crate::{PixelsSendMode, pixels_string::*};

const DISPLAY_RESULTS: bool = true;

#[test]
fn string_of_chars() {
    create_dir_recursive("fonts/exports/").unwrap();

    //let image_transparent_bkgrnd = PixelsCollection::from_png("fonts/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
    let image_transparent_bkgrnd = PixelsCollection::<u8>::from_png(
        "fonts/transparent_green_40px_chars_sample__transparent_background.png",
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
    let img_visible_range = match get_cardinal_points_until_nonestreak_x(
        &buffer,
        height,
        start_x,
        start_y,
        range_x,
        range_y,
        range_x,
        |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
    ) {
        Some(c_p) => c_p,
        None => panic!(
            "Could not set img_visible_range. No pixels matching the bgra_matcher found in provided range."
        ),
    };

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

    let mut char_u8_vec: CharsCollection<u8> =
        CharsCollection { chars: Vec::new(), path: "".to_string(), bgra: BGRA(0, 0, 0, 255) };

    let mut bytes_chars_poles = buffer.clone();

    for char in chars_string.chars() {
        let values = match get_cardinal_points_until_nonestreak_x(
            &buffer,
            height,
            start_x,
            start_y,
            range_x,
            range_y,
            min_px_space_btwn_chars,
            |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
        ) {
            Some(c_p) => c_p,
            None => panic!("No pixels matching the bgra_matcher found in provided range."),
        };

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
            bytes_chars_poles.set_positions_bgra(height, &vec_pos_char_strict, 170, 255, 170, 255);
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
                (char_values.right_x, values.bottom_y - img_visible_range.top_y),
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
            pixels_captured_clone.set_addresses_bgra(&vec_addresses_char_relative, 0, 0, 255, 255);
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
    create_dir_recursive("fonts/exports/").unwrap();

    //let image_transparent_bkgrnd = PixelsCollection::from_png("fonts/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
    let image_transparent_bkgrnd = PixelsCollection::<u8>::from_png(
        "fonts/transparent_green_40px_chars_sample__transparent_background.png",
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
    let img_visible_range = match get_cardinal_points_until_nonestreak_x(
        &buffer,
        height,
        start_x,
        start_y,
        range_x,
        range_y,
        range_x,
        |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
    ) {
        Some(c_p) => c_p,
        None => panic!(
            "Could not set img_visible_range. No pixels matching the bgra_matcher found in provided range."
        ),
    };

    let original_range_x = range_x;

    let mut bytes_chars_poles = buffer.clone();

    let mut highest_height = 0;

    for char in chars_string.chars() {
        let values = match get_cardinal_points_until_nonestreak_x(
            &buffer,
            height,
            start_x,
            start_y,
            range_x,
            range_y,
            min_px_space_btwn_chars,
            |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
        ) {
            Some(c_p) => c_p,
            None => panic!("No pixels matching the bgra_matcher found in provided range."),
        };

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
            bytes_chars_poles.set_positions_bgra(height, &vec_pos_char_strict, 170, 255, 170, 255);
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
        let values = match get_cardinal_points_until_nonestreak_x(
            &buffer,
            height,
            start_x,
            start_y,
            range_x,
            range_y,
            min_px_space_btwn_chars,
            |_: u8, _: u8, _: u8, a: u8| -> bool { a > 0 },
        ) {
            Some(c_p) => c_p,
            None => panic!("No pixels matching the bgra_matcher found in provided range."),
        };

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
