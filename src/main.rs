pub use pixel_caster::*;

/// main() will just get and send some bytes to verify that the screen reacts to the new values
fn main() {

    println!("");
    println!("Just playing with some pixels, to verify that the screen reacts to the new values (check the top-left corner)");

    let mut screen_area_to_capture_upperleftcorner_x = 60;
    let mut screen_area_to_capture_upperleftcorner_y = 60;
    let mut pixels_width = 80;
    let mut pixels_height = 80;
    let mut screen_destination_area_upperleftcorner_x = 30;
    let mut screen_destination_area_upperleftcorner_y = 90;
    copy_and_paste_pixels(
        &pixels_width,
        &pixels_height,
        &screen_area_to_capture_upperleftcorner_x,
        &screen_area_to_capture_upperleftcorner_y,
        &screen_destination_area_upperleftcorner_x,
        &screen_destination_area_upperleftcorner_y
    );
    
    screen_area_to_capture_upperleftcorner_x = 420;
    screen_area_to_capture_upperleftcorner_y = 110;
    pixels_width = 100;
    pixels_height = 100;
    screen_destination_area_upperleftcorner_x = 200;
    screen_destination_area_upperleftcorner_y = 200;


    // get the bytes from the pixels of the requested size from an absolute position on the screen
    let mut vec = get_bytes(
        &pixels_width,
        &pixels_height,
        &screen_area_to_capture_upperleftcorner_x, 
        &screen_area_to_capture_upperleftcorner_y
    );
    /* Vec for testing : a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue
    let mut vec:Vec<u8> = Vec::with_capacity(4 * 4 * 4);
    vec.extend_from_slice(&[0,0,255,255,0,0,255,255]);
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]);
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]);*/

    // send the bytes to the pixels of the requested size of an absolute position on the screen
    send_bytes(
        &mut vec,
        &pixels_width,
        &pixels_height,
        &screen_destination_area_upperleftcorner_x, 
        &screen_destination_area_upperleftcorner_y
    );


}
// based on
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen