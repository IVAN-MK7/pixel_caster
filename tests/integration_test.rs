use pixel_caster::bgra_management::{bytes_matchers, SwitchBytes};
use pixel_caster::{pixels_string::*, PixelsCollection, *};

#[test]
/// Gets a PixelsCollection from a .png, then attempts to create a CharsCollection from it
fn png_to_char_collection() {
    let image = PixelsCollection::<u8>::from_png(
        "fonts/exports/transparent_green_40px_chars_sample__transparent_background.png",
    )
    .unwrap();

    let transparent_green_chars_transparent_background = image.try_create_char_collection(
        10,
        r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#,
        5,
        bytes_matchers::visible,
    );

    match transparent_green_chars_transparent_background {
        Ok(transparent_green_chars_transparent_background) => {
            // create a string and print it on the screen
            let transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground =
                transparent_green_chars_transparent_background
                    .create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(
                &transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.bytes,
                10, 10,
                transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.width as u32,
                transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.height as u32,
                PixelsSendMode::AlphaEnabled
            );
            // export the string as .png
            // image::save_buffer_with_format(format!("{}{}", "fonts/exports/", "test_improvement.png"), &vec_u8_managing::<u8>::swap_blue_with_red(&string_from_string_png.pixels.bytes), string_from_string_png.pixels.width as u32, string_from_string_png.pixels.height as u32, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();

            transparent_green_chars_transparent_background.export_as_pngs("fonts/exports/from_transparent_green_40px_chars_sample__transparent_background/transparent_green__transparent_background").unwrap();

            // export the first char as .png
            // image::save_buffer_with_format(format!("{}{}", "fonts/exports/", "first_char_export.png"), &vec_u8_managing::<u8>::swap_blue_with_red(&transparent_green_chars_transparent_background.chars[0].pixels.bytes), transparent_green_chars_transparent_background.chars[0].pixels.width as u32, transparent_green_chars_transparent_background.chars[0].pixels.height as u32, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();
        }
        Err(err) => print!("{}", err),
    }

    let image = PixelsCollection::<u8>::from_png(
        "fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png",
    )
    .unwrap();

    let opaque_grey_scale_chars_white_background = image.try_create_char_collection(
        6,
        r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#,
        5,
        bytes_matchers::visible_not_white,
    );

    match opaque_grey_scale_chars_white_background {
        Ok(mut opaque_grey_scale_chars_white_background) => {
            opaque_grey_scale_chars_white_background.set_bgra_for_invisible(255, 255, 255, 255); // make the invisible pixels white

            // create a string and print it on the screen
            let mut string_from_string_png = opaque_grey_scale_chars_white_background
                .create_pixels_string("testing generated_text!^", 3);
            string_from_string_png
                .pixels
                .set_bgra_for_invisible(255, 255, 255, 255); // make also the spaces between the chars white
            Screen::update_area_custom(
                &string_from_string_png.pixels.bytes,
                10,
                80,
                string_from_string_png.pixels.width as u32,
                string_from_string_png.pixels.height as u32,
                PixelsSendMode::AlphaEnabled,
            );

            opaque_grey_scale_chars_white_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/opaque_grey_scale__white_background").unwrap();
        }
        Err(err) => print!("{}", err),
    }

    let mut image = PixelsCollection::<u8>::from_png(
        "fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png",
    )
    .unwrap();
    image.bytes = PixelsCollection::white_background_to_transparency_gradient(&image.bytes);
    image.set_bgr(0, 0, 0); // set the color to black

    let transparent_black_chars_transparent_background = image.try_create_char_collection(
        6,
        r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#,
        5,
        bytes_matchers::visible,
    );

    match transparent_black_chars_transparent_background {
        Ok(transparent_black_chars_transparent_background) => {
            // create a string and print it on the screen
            let string_from_string_png = transparent_black_chars_transparent_background
                .create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(
                &string_from_string_png.pixels.bytes,
                10,
                110,
                string_from_string_png.pixels.width as u32,
                string_from_string_png.pixels.height as u32,
                PixelsSendMode::AlphaEnabled,
            );

            transparent_black_chars_transparent_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/transparent_black__transparent_background").unwrap();
        }
        Err(err) => print!("{}", err),
    }

    let mut image = PixelsCollection::<u8>::from_png(
        "fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png",
    )
    .unwrap();
    PixelsCollection::grey_scale_into_black(&mut image.bytes, 200);

    let threshold_black_chars_transparent_background = image.try_create_char_collection(
        6,
        r#"abcdefghijklmnopqrstuvwxyz,.?!0123456789-+/*\_@#()[]{};:"£$%&='^"#,
        5,
        bytes_matchers::visible,
    );

    match threshold_black_chars_transparent_background {
        Ok(threshold_black_chars_transparent_background) => {
            // create a string and print it on the screen
            let string_from_string_png = threshold_black_chars_transparent_background
                .create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(
                &string_from_string_png.pixels.bytes,
                10,
                140,
                string_from_string_png.pixels.width as u32,
                string_from_string_png.pixels.height as u32,
                PixelsSendMode::AlphaEnabled,
            );

            threshold_black_chars_transparent_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__transparent_background").unwrap();

            let mut threshold_black_chars_white_background =
                threshold_black_chars_transparent_background.clone();
            threshold_black_chars_white_background.set_bgra_for_invisible(255, 255, 255, 255);
            threshold_black_chars_white_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__white_background").unwrap();
        }
        Err(err) => print!("{}", err),
    }
}

/// Attempts to create a CharsCollection from a folder of .png, then creates and alterates its clones
#[test]
fn char_collection_alteration() {
    let threshold_black_chars_transparent_background = CharsCollection::from_pngs_folder("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__transparent_background").unwrap();

    let pixels_string_black_transparent_background = threshold_black_chars_transparent_background
        .create_pixels_string("black test text with transparent background !", 1);
    crate::Screen::update_area_custom(
        &pixels_string_black_transparent_background.pixels.bytes,
        0,
        0,
        pixels_string_black_transparent_background.pixels.width as u32,
        pixels_string_black_transparent_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled,
    );

    let mut threshold_black_chars_white_background =
        threshold_black_chars_transparent_background.clone();
    threshold_black_chars_white_background.set_bgra_for_invisible(255, 255, 255, 255); // make the invisible pixels white

    let mut pixels_string_black_white_background = threshold_black_chars_white_background
        .create_pixels_string("black test text with white background !", 1);
    pixels_string_black_white_background
        .pixels
        .set_bgra_for_invisible(255, 255, 255, 255); // make also the spaces between the chars white

    crate::Screen::update_area_custom(
        &pixels_string_black_white_background.pixels.bytes,
        0,
        pixels_string_black_transparent_background.pixels.height as i32,
        pixels_string_black_white_background.pixels.width as u32,
        pixels_string_black_white_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled,
    );

    threshold_black_chars_white_background.export_as_pngs("fonts/exports/from_threshold_black__transparent_background/threshold_black_chars__white_background").unwrap();

    let mut transparent_black_chars_transparent_background = CharsCollection::from_pngs_folder("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/transparent_black__transparent_background").unwrap();
    transparent_black_chars_transparent_background.set_bgr(0, 0, 255);

    let pixels_string_red_transparent_background = transparent_black_chars_transparent_background
        .create_pixels_string("red test text with transparent background !", 1);
    crate::Screen::update_area_custom(
        &pixels_string_red_transparent_background.pixels.bytes,
        0,
        pixels_string_black_transparent_background.pixels.height as i32 * 2,
        pixels_string_red_transparent_background.pixels.width as u32,
        pixels_string_red_transparent_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled,
    );

    transparent_black_chars_transparent_background.export_as_pngs("fonts/exports/from_transparent_black__transparent_background/transparent_red_chars__transparent_background").unwrap();
}

#[test]
fn copy_and_paste_pixels() {
    let screen_area_to_capture_upperleftcorner_x = 100;
    let screen_area_to_capture_upperleftcorner_y = 100;
    let pixels_width = 100;
    let pixels_height = 100;

    // u8 Screen variant
    let mut screen_u8: Screen<u8> = Screen::new(80, 2, pixels_width, pixels_height);
    screen_u8.scan_area();

    // send the bytes to the pixels of the requested size of an absolute position on the screen
    Screen::update_area_custom(
        screen_u8.get_bytes(),
        screen_area_to_capture_upperleftcorner_x + 100,
        screen_area_to_capture_upperleftcorner_y + 100,
        pixels_width,
        pixels_height,
        PixelsSendMode::AlphaEnabled,
    );

    println!("Each pixel's color is obtained by its BGRA values combination, in a Vector of u8 those 4 values occupy 1 position each, in a Vector of u32 those 4 values occupy together just one position.");
    println!("To contain the BGRA (Blue, Green, Red, Alpha) values of a single pixel a Vec<u8> would have a lenght of 4, a Vec<u32> would have a lenght of just 1");
    println!("The provided screen area (starting at X: {}, Y: {}) has a width of: {}px and a height of: {}px, for a total pixel count of {}", screen_area_to_capture_upperleftcorner_x, screen_area_to_capture_upperleftcorner_y, pixels_width, pixels_height, pixels_width * pixels_height);
    println!("The BGRA values of the given screen area have been retrieved both into a Vec<u8> and a Vec<u32>");
    println!(
        "screen_u8.bytes: first value: {}, length: {}",
        screen_u8.get_bytes()[0],
        screen_u8.get_bytes().len()
    );
    println!(
        "screen_u8.bytes: first pixel's values: B:{} G:{} R:{} A:{}",
        screen_u8.get_bytes()[0],
        screen_u8.get_bytes()[1],
        screen_u8.get_bytes()[2],
        screen_u8.get_bytes()[3]
    );

    // u32 Screen variant (the variant can also be specified using the turbofish ::<>, as in this case)
    let mut screen_u32 = Screen::<u32>::new(80, 2, 4, 1);
    screen_u32.scan_area();
    let vec_u32_first_value = screen_u32.get_bytes()[0];
    let bgra = <u32>::u8_u32_casting(&[vec_u32_first_value]);
    println!(
        "screen_u32.bytes: first value: {}, length: {}",
        vec_u32_first_value,
        screen_u32.get_bytes().len()
    );
    println!(
        "screen_u32.bytes: first pixel's values: B:{} G:{} R:{} A:{}",
        bgra[0], bgra[1], bgra[2], bgra[3]
    );
}

#[test]
fn copy_and_paste_pixels_slimmed() {
    let screen_area_to_capture_upperleftcorner_x = 100;
    let screen_area_to_capture_upperleftcorner_y = 100;
    let pixels_width = 400;
    let pixels_height = 400;

    // u8 Screen variant
    let mut screen_u8: Screen<u8> = Screen::new(80, 2, pixels_width, pixels_height);
    screen_u8.scan_area();

    let mut screen_u8_dst: Screen<u8> = Screen::new(
        screen_area_to_capture_upperleftcorner_x + 100,
        screen_area_to_capture_upperleftcorner_y + 100,
        pixels_width,
        pixels_height,
    );
    screen_u8_dst.update_area_from_vec(screen_u8.get_bytes());
    screen_u8.scan_area();
    screen_u8_dst.update_area_from_vec(screen_u8.get_bytes());

    println!("Each pixel's color is obtained by its BGRA values combination, in a Vector of u8 those 4 values occupy 1 position each, in a Vector of u32 those 4 values occupy together just one position.");
    println!("To contain the BGRA (Blue, Green, Red, Alpha) values of a single pixel a Vec<u8> would have a lenght of 4, a Vec<u32> would have a lenght of just 1");
    println!("The provided screen area (starting at X: {}, Y: {}) has a width of: {}px and a height of: {}px, for a total pixel count of {}", screen_area_to_capture_upperleftcorner_x, screen_area_to_capture_upperleftcorner_y, pixels_width, pixels_height, pixels_width * pixels_height);
    println!("The BGRA values of the given screen area have been retrieved both into a Vec<u8> and a Vec<u32>");
    println!(
        "screen_u8.bytes: first value: {}, length: {}",
        screen_u8.get_bytes()[0],
        screen_u8.get_bytes().len()
    );
    println!(
        "screen_u8.bytes: first pixel's values: B:{} G:{} R:{} A:{}",
        screen_u8.get_bytes()[0],
        screen_u8.get_bytes()[1],
        screen_u8.get_bytes()[2],
        screen_u8.get_bytes()[3]
    );

    // u32 Screen variant (the variant can also be specified using the turbofish ::<>, as in this case)
    let mut screen_u32 = Screen::<u32>::new(80, 2, 4, 1);
    screen_u32.scan_area();
    let vec_u32_first_value = screen_u32.get_bytes()[0];
    let bgra = <u32>::u8_u32_casting(&[vec_u32_first_value]);
    println!(
        "screen_u32.bytes: first value: {}, length: {}",
        vec_u32_first_value,
        screen_u32.get_bytes().len()
    );
    println!(
        "screen_u32.bytes: first pixel's values: B:{} G:{} R:{} A:{}",
        bgra[0], bgra[1], bgra[2], bgra[3]
    );
}

#[test]
fn test_send_bytes_transparency() {
    let image_u8_bgra =
        PixelsCollection::<u8>::from_png("media/Logo_MK7_Transparent_Bg_ColorsWithHalfAlpha.png")
            .unwrap();
    Screen::update_area_custom(
        &image_u8_bgra.bytes,
        0,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaEnabled,
    );
    Screen::update_area_custom(
        &image_u8_bgra.bytes,
        0,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaDisabled,
    );
    Screen::update_area_custom(
        &image_u8_bgra.bytes,
        0,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaDisabledHideBGR(0, 0, 255),
    );
    Screen::update_area_custom(
        &image_u8_bgra.bytes,
        0,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::CustomAlpha(100),
    );
}

#[test]
fn test_get_bytes() {
    let screen_area_upperleftcorner_x = 100;
    let screen_area_upperleftcorner_y = 100;
    let area_width = 200;
    let area_height = 200;

    // The provided Vec must already have the necessary length to host the values.
    let mut bytes = <u8 as PixelValues<u8>>::initialize_vec(area_width, area_height);

    let screen = Screen::<u8>::new(
        screen_area_upperleftcorner_x,
        screen_area_upperleftcorner_y,
        area_width as u32,
        area_height as u32,
    );
    screen.scan_area_onto_vec(&mut bytes).unwrap();

    Screen::update_area_custom(
        &bytes,
        10,
        10,
        area_width as u32,
        area_height as u32,
        pixel_caster::PixelsSendMode::AlphaEnabled,
    );
}
