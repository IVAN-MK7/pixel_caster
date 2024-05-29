pub use pixel_caster::*;

fn main() {
    let screen_destination_area_upperleftcorner_x = 30;
    let screen_destination_area_upperleftcorner_y = 90;
    let pixels_area_width = 4;
    let pixels_area_height = 4;

    /* Vec for testing : a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue*/
    let mut vec: Vec<u8> = Vec::with_capacity(pixels_area_width * pixels_area_height * 4);
    // each byte (u8) has a range of 0-255, they are ordered by BGRA instead of RGBA
    vec.extend_from_slice(&[0, 0, 255, 255, /**/ 0, 0, 246, 245]); // 2 RED pixels
    vec.extend_from_slice(&[
        255, 0, 0, 125, /**/ 255, 0, 0, 160, /**/ 255, 0, 0, 190, /**/
        255, 0, 0, 210, /**/ 255, 0, 0, 230, /**/ 255, 0, 0, 245, /**/
        255, 0, 0, 255,
    ]); // 7 BLUE pixels with various Alpha values (in order to make them differ in opacity/transparency)
    vec.extend_from_slice(&[
        255, 0, 0, 0, /**/ 255, 0, 0, 125, /**/ 255, 0, 0, 255, /**/
        255, 0, 0, 255, /**/ 255, 0, 0, 255, /**/ 255, 0, 0, 255, /**/
        255, 0, 0, 255,
    ]); // 7 more

    // sets the BGRA sending method to AlphaEnabled in to use per-pixel alpha values
    let pixels_send_mode = PixelsSendMode::AlphaEnabled;
    // send the bytes to the pixels of the requested size of an absolute position on the screen
    Screen::update_area_custom(
        &vec,
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
        pixels_area_width as u32,
        pixels_area_height as u32,
        pixels_send_mode,
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
