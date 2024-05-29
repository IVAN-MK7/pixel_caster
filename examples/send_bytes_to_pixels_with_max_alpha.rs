pub use pixel_caster::*;

fn main() {
    let screen_destination_area_upperleftcorner_x = 30;
    let screen_destination_area_upperleftcorner_y = 90;
    let pixels_area_width = 4;
    let pixels_area_height = 4;

    // Vec for testing : a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue
    let mut vec: Vec<u8> = Vec::with_capacity(pixels_area_width * pixels_area_height * 4);
    // each byte (u8) has a range of 0-255, they are ordered in BGRA
    //                      B  G  R    A
    vec.extend_from_slice(&[0, 0, 255, 0]); // 1 RED pixel
    vec.extend_from_slice(&[0, 0, 255, 125]); // 1 RED pixel
    vec.extend_from_slice(&[
        255, 0, 0, 100, /**/ 255, 0, 0, 200, /**/ 255, 0, 0, 120, /**/
        255, 0, 0, 200, /**/ 255, 0, 0, 180, /**/ 255, 0, 0, 255, /**/
        255, 0, 0, 100,
    ]); // 7 BLUE pixels
    vec.extend_from_slice(&[
        255, 0, 0, 200, /**/ 255, 0, 0, 0, /**/ 255, 0, 0, 10, /**/ 255, 0, 0, 100,
        255, 0, 0, 225, /**/ 255, 0, 0, 255, /**/ 255, 0, 0, 255,
    ]); // 7 BLUE pixels

    // send the bytes to the pixels of the requested size of an absolute position on the screen
    // each pixel's Alpha value can be any value, since with the following function every pixel will have Alpha automatically set to 255
    // therefore the pixels Alpha channel (trasparency) is disabled, so each pixel has full opacity (Alpha = 255)
    Screen::update_area_custom(
        &vec,
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
        pixels_area_width as u32,
        pixels_area_height as u32,
        PixelsSendMode::AlphaDisabled,
    );

    println!("\r\nA Vec<u8> containing 64 bytes, representing a qube of 4 x 4 (16) pixels, where the first 2 are red, the other 14 blue, will now be sent to the screen at the location :");
    println!(
        "X : {}, Y : {}",
        screen_destination_area_upperleftcorner_x, screen_destination_area_upperleftcorner_y
    );
    println!(
        "The first pixel has the following RGBA values : R:{} G:{} B:{} A:{}",
        vec[2], vec[1], vec[0], vec[3]
    );
}
