use pixel_caster::{*, pixels_string::{PixelsCollection,CharsCollection}};
use pixel_caster::bgra_management::bytes_matchers;
fn main () {
    // starting from a .png containing a sample of characters

    let image = pixels_string::PixelsCollection::<u8>::from_png("fonts/exports/transparent_green_40px_chars_sample__transparent_background.png").unwrap();
    
    let transparent_green_chars_transparent_background = image.try_create_char_collection(10, r#"abcdefghijklmnopqrstuvwxyz,.?!01234567890-+/*\_@#()[]{}"£$%&='^"#, 5, bytes_matchers::visible);

    match transparent_green_chars_transparent_background {
        Ok(transparent_green_chars_transparent_background) => {
            // create a string and print it on the screen
            let transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground = transparent_green_chars_transparent_background.create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(
                &transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.bytes,
                10, 10,
                transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.width as u32,
                transparent_green_string_transparent_background_from_chars_sample_40px_green_whitebackground.pixels.height as u32,
                PixelsSendMode::AlphaEnabled
            );
            // export the string as .png
            // image::save_buffer_with_format(format!("{}{}", "fonts/exports/", "test_improvement.png"), &vec_u8_managing::bgra_u8_to_rgba_u8(&string_from_string_png.pixels.bytes), string_from_string_png.pixels.width as u32, string_from_string_png.pixels.height as u32, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();        
            
            transparent_green_chars_transparent_background.export_as_pngs("fonts/exports/from_transparent_green_40px_chars_sample__transparent_background/transparent_green__transparent_background").unwrap();
            
            // export the first char as .png
            // image::save_buffer_with_format(format!("{}{}", "fonts/exports/", "first_char_export.png"), &vec_u8_managing::bgra_u8_to_rgba_u8(&transparent_green_chars_transparent_background.chars[0].pixels.bytes), transparent_green_chars_transparent_background.chars[0].pixels.width as u32, transparent_green_chars_transparent_background.chars[0].pixels.height as u32, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();        
        },
        Err(err) => print!("{}", err),
    }
    

    let image = pixels_string::PixelsCollection::<u8>::from_png("fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
    
    let opaque_grey_scale_chars_white_background = image.try_create_char_collection(6, r#"abcdefghijklmnopqrstuvwxyz,.?!01234567890-+/*\_@#()[]{}"£$%&='^"#, 5, bytes_matchers::visible_not_white);

    match opaque_grey_scale_chars_white_background {
        Ok(mut opaque_grey_scale_chars_white_background) => {
            opaque_grey_scale_chars_white_background.set_bgra_for_invisible(255,255,255,255); // make the invisible pixels white

            // create a string and print it on the screen
            let mut string_from_string_png = opaque_grey_scale_chars_white_background.create_pixels_string("testing generated_text!^", 3);
            string_from_string_png.pixels.set_bgra_for_invisible(255,255,255,255); // make also the spaces between the chars white
            Screen::update_area_custom(&string_from_string_png.pixels.bytes, 10, 80, string_from_string_png.pixels.width as u32, string_from_string_png.pixels.height as u32, PixelsSendMode::AlphaEnabled);
            
            opaque_grey_scale_chars_white_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/opaque_grey_scale__white_background").unwrap();
        },
        Err(err) => print!("{}", err),
    }



    let mut image = pixels_string::PixelsCollection::<u8>::from_png("fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
    image.bytes = PixelsCollection::white_background_to_transparency_gradient(&image.bytes);

    let transparent_black_chars_transparent_background = image.try_create_char_collection(6, r#"abcdefghijklmnopqrstuvwxyz,.?!01234567890-+/*\_@#()[]{}"£$%&='^"#, 5, bytes_matchers::visible);

    match transparent_black_chars_transparent_background {
        Ok(transparent_black_chars_transparent_background) => {
            // create a string and print it on the screen
            let string_from_string_png = transparent_black_chars_transparent_background.create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(&string_from_string_png.pixels.bytes, 10, 110,string_from_string_png.pixels.width as u32, string_from_string_png.pixels.height as u32, PixelsSendMode::AlphaEnabled);
            
            transparent_black_chars_transparent_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/transparent_black__transparent_background").unwrap();
        },
        Err(err) => print!("{}", err),
    }
    
    let mut image = pixels_string::PixelsCollection::<u8>::from_png("fonts/exports/opaque_grey_scale_12px_chars_sample__white_background.png").unwrap();
    PixelsCollection::grey_scale_into_black(&mut image.bytes, 200);

    let threshold_black_chars_transparent_background = image.try_create_char_collection(6, r#"abcdefghijklmnopqrstuvwxyz,.?!01234567890-+/*\_@#()[]{}"£$%&='^"#, 5, bytes_matchers::visible);

    match threshold_black_chars_transparent_background {
        Ok(threshold_black_chars_transparent_background) => {
            // create a string and print it on the screen
            let string_from_string_png = threshold_black_chars_transparent_background.create_pixels_string("testing generated_text!^", 3);
            Screen::update_area_custom(&string_from_string_png.pixels.bytes, 10, 140, string_from_string_png.pixels.width as u32, string_from_string_png.pixels.height as u32, PixelsSendMode::AlphaEnabled);
            
            threshold_black_chars_transparent_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__transparent_background").unwrap();

            let mut threshold_black_chars_white_background = threshold_black_chars_transparent_background.clone();
            threshold_black_chars_white_background.set_bgra_for_invisible(255,255,255,255);
            threshold_black_chars_white_background.export_as_pngs("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__white_background").unwrap();
        },
        Err(err) => print!("{}", err),
    }
    




    // starting from a folder of .png files, each containing a char
    

    let threshold_black_chars_transparent_background = CharsCollection::from_pngs_folder("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/threshold_black__transparent_background").unwrap();

    let pixels_string_black_transparent_background = threshold_black_chars_transparent_background.create_pixels_string("black test text with transparent background !", 1);
    crate::Screen::update_area_custom(
        &pixels_string_black_transparent_background.pixels.bytes,
        0, 150,
        pixels_string_black_transparent_background.pixels.width as u32,
        pixels_string_black_transparent_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled
    );

    
    let mut threshold_black_chars_white_background = threshold_black_chars_transparent_background.clone();
    threshold_black_chars_white_background.set_bgra_for_invisible(255,255,255,255); // make the invisible pixels white

    let mut pixels_string_black_white_background = threshold_black_chars_white_background.create_pixels_string("black test text with white background !", 1);
    pixels_string_black_white_background.pixels.set_bgra_for_invisible(255,255,255,255); // make also the spaces between the chars white

    crate::Screen::update_area_custom(
        &pixels_string_black_white_background.pixels.bytes,
        0, 150 + 10 + pixels_string_black_transparent_background.pixels.height as i32,
        pixels_string_black_white_background.pixels.width as u32,
        pixels_string_black_white_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled
    );

    threshold_black_chars_white_background.export_as_pngs("fonts/exports/from_threshold_black__transparent_background/threshold_black_chars__white_background").unwrap();


    let mut transparent_black_chars_transparent_background = CharsCollection::from_pngs_folder("fonts/exports/from_opaque_grey_scale_12px_chars_sample__white_background/transparent_black__transparent_background").unwrap();
    transparent_black_chars_transparent_background.set_bgr(0,0,255);

    let pixels_string_red_transparent_background = transparent_black_chars_transparent_background.create_pixels_string("red test text with transparent background !", 1);
    crate::Screen::update_area_custom(
        &pixels_string_red_transparent_background.pixels.bytes,
        0, 150 + 10 + pixels_string_black_transparent_background.pixels.height as i32 * 2,
        pixels_string_red_transparent_background.pixels.width as u32,
        pixels_string_red_transparent_background.pixels.height as u32,
        PixelsSendMode::AlphaEnabled
    );
    
    transparent_black_chars_transparent_background.export_as_pngs("fonts/exports/from_transparent_black__transparent_background/transparent_red_chars__transparent_background").unwrap();


}