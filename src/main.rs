use pixel_caster::*;
/// main() will just get and send some bytes to verify that the screen reacts to the new values
fn main() {
    println!("\r\nJust playing with some pixels, to verify that the screen reacts to the new values (check the top-left corner)");

    let mut screen_area_to_capture_upperleftcorner_x = 60;
    let mut screen_area_to_capture_upperleftcorner_y = 60;
    let mut pixels_width = 80;
    let mut pixels_height = 80;
    let mut screen_destination_area_upperleftcorner_x = 30;
    let mut screen_destination_area_upperleftcorner_y = 90;
    Screen::<u8>::copy_and_paste_pixels(
        screen_area_to_capture_upperleftcorner_x,
        screen_area_to_capture_upperleftcorner_y,
        pixels_width,
        pixels_height,
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
    );

    screen_area_to_capture_upperleftcorner_x = 420;
    screen_area_to_capture_upperleftcorner_y = 110;
    pixels_width = 100;
    pixels_height = 100;
    screen_destination_area_upperleftcorner_x = 200;
    screen_destination_area_upperleftcorner_y = 200;

    // get the bytes from the pixels of the requested size from an absolute position on the screen
    // u8 Screen variant
    let mut screen_u8: Screen<u8> = Screen::new(
        screen_area_to_capture_upperleftcorner_x,
        screen_area_to_capture_upperleftcorner_y,
        pixels_width,
        pixels_height,
    );
    screen_u8.scan_area();
    /* Vec for testing : a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue
    let mut vec:Vec<u8> = Vec::with_capacity(4 * 4 * 4);
    vec.extend_from_slice(&[0,0,255,255,0,0,255,255]);
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]);
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]);*/

    // send the bytes to the pixels of the requested size of an absolute position on the screen,
    // makes completery transparent those whites (obtained by R: 255, G: 255, B: 255) that are being sent to the screen's pixels
    Screen::update_area_custom(
        screen_u8.get_bytes(),
        screen_destination_area_upperleftcorner_x + 220,
        screen_destination_area_upperleftcorner_y,
        pixels_width,
        pixels_height,
        PixelsSendMode::AlphaDisabledHideBGR(255, 255, 255),
    );

    // sets the BGRA sending method to AlphaEnabled in to use per-pixel alpha values
    let pixels_send_mode = PixelsSendMode::AlphaEnabled;
    // send the bytes to the pixels of the requested size of an absolute position on the screen
    Screen::update_area_custom(
        screen_u8.get_bytes(),
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
        pixels_width,
        pixels_height,
        pixels_send_mode,
    );
}
// based on
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen
