#![cfg(test)]

use crate::{
    PixelsCollection, PixelsSendMode, Screen, bgra_management::*,
    pixels_string::create_dir_recursive,
};

#[test]
fn test_u8_u32_convertion() {
    create_dir_recursive("media/exports/").unwrap();

    let mut bytes_u8_bgra: Vec<u8> = Vec::new();
    //                                B    G  R   A
    //                                blue
    bytes_u8_bgra.extend_from_slice(&[255, 0, 0, 125]);
    //                                  green
    bytes_u8_bgra.extend_from_slice(&[0, 255, 0, 125]);
    //                                      red
    bytes_u8_bgra.extend_from_slice(&[0, 0, 255, 125]);

    Screen::update_area_custom(
        &bytes_u8_bgra,
        0,
        0,
        bytes_u8_bgra.len() as u32 / 4,
        1,
        PixelsSendMode::AlphaEnabled,
    );
    // when exporting into .ong we need to go from BGRA to RGBA, so swap the B and R values
    let mut bytes_u8_rgba_from_u8_bgra = bytes_u8_bgra.clone();
    <u8>::switch_bytes(&mut bytes_u8_rgba_from_u8_bgra, 0, 2);
    image::save_buffer_with_format(
        format!("{}{}", "media/exports/", "bytes_u8_rgba_from_u8_bgra.png"),
        &bytes_u8_rgba_from_u8_bgra,
        (bytes_u8_rgba_from_u8_bgra.len() / 4).try_into().unwrap(),
        1,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();

    // u8 [B,G,R,A] -> u32 on little endian CPU the order becomes becomes : [0xARGB]
    let bytes_u32_bgra = <u8>::u8_u32_casting(&bytes_u8_bgra);
    // A = 125, R = 0, G = 0, B = 255
    assert_eq!(bytes_u32_bgra[0], 0x7D00_00FF);
    // on big endian CPU we would have compared it to 0xFF00_007D, because the order would be [0xBGRA]
    Screen::update_area_custom(
        &bytes_u32_bgra,
        0,
        0,
        bytes_u32_bgra.len() as u32,
        1,
        PixelsSendMode::AlphaEnabled,
    );

    let mut bytes_u32_rgba_from_u32_bgra = bytes_u32_bgra.clone();
    <u32>::switch_bytes(&mut bytes_u32_rgba_from_u32_bgra, 0, 2);
    // A = 125, R = 255, G = 0, B = 0 , because we switched the value of B (index 0 in BGRA) with R (index 2 in BGRA)
    assert_eq!(bytes_u32_rgba_from_u32_bgra[0], 0x7DFF_0000);
    // on big endian CPU we would have compared it to 0x0000_FF7D, because the order would be [0xBGRA]

    let bytes_u8_rgba_from_u32_rgba = <u32>::u8_u32_casting(&bytes_u32_rgba_from_u32_bgra);
    // when exporting into .ong we need RGBA values
    image::save_buffer_with_format(
        format!("{}{}", "media/exports/", "bytes_u8_rgba_from_u32_rgba.png"),
        &bytes_u8_rgba_from_u32_rgba,
        (bytes_u8_rgba_from_u32_rgba.len() / 4).try_into().unwrap(),
        1,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();
}

#[test]
fn test_u8_u32_convertion_png_with_transparency() {
    create_dir_recursive("media/exports/").unwrap();

    let image_u8_bgra =
        PixelsCollection::<u8>::from_png("media/Logo_MK7_Transparent_Bg_ColorsWithHalfAlpha.png")
            .unwrap();

    let image_u8_bgra_from_image_rgba = image_u8_bgra.clone();
    Screen::update_area_custom(
        &image_u8_bgra_from_image_rgba.bytes,
        -200,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaEnabled,
    );
    // when exporting into .png we need to go from BGRA to RGBA, so swap the B and R values
    let mut image_u8_rgba_from_image_u8_bgra = image_u8_bgra_from_image_rgba.clone();
    image_u8_rgba_from_image_u8_bgra.switch_bytes(0, 2);
    image::save_buffer_with_format(
        format!("{}{}", "media/exports/", "rgba_u8.png"),
        &image_u8_rgba_from_image_u8_bgra.bytes,
        image_u8_bgra.width.try_into().unwrap(),
        image_u8_bgra.height.try_into().unwrap(),
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();

    let rgba_u32 = <u8>::u8_u32_casting(&image_u8_bgra.bytes);
    let image_u32_rgba =
        PixelsCollection::<u32>::create(image_u8_bgra.width, image_u8_bgra.height, rgba_u32)
            .unwrap();
    Screen::update_area_custom(
        &image_u32_rgba.bytes,
        -200,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaEnabled,
    );

    // image_u32_rgba has BGRA ordered bytes
    let mut bytes_u8_bgra_from_u32_bgra = <u32>::u8_u32_casting(&image_u32_rgba.bytes);
    Screen::update_area_custom(
        &bytes_u8_bgra_from_u32_bgra,
        -200,
        0,
        image_u8_bgra.width as u32,
        image_u8_bgra.height as u32,
        PixelsSendMode::AlphaEnabled,
    );
    // when exporting into .png we need to go from BGRA to RGBA, so swap the B and R values
    <u8>::switch_bytes(&mut bytes_u8_bgra_from_u32_bgra, 0, 2);
    image::save_buffer_with_format(
        format!("{}{}", "media/exports/", "bytes_u8_rgba_from_u32_rgba_with_transparency.png"),
        &bytes_u8_bgra_from_u32_bgra,
        image_u8_bgra.width.try_into().unwrap(),
        image_u8_bgra.height.try_into().unwrap(),
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();
}
