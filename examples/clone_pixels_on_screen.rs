pub use pixel_caster::*;

fn main() {
    let screen_area_to_capture_upperleftcorner_x = 60;
    let screen_area_to_capture_upperleftcorner_y = 60;
    let pixels_width = 200;
    let pixels_height = 400;
    let screen_destination_area_upperleftcorner_x = 30;
    let screen_destination_area_upperleftcorner_y = 90;
    Screen::<u8>::copy_and_paste_pixels(
        screen_area_to_capture_upperleftcorner_x,
        screen_area_to_capture_upperleftcorner_y,
        pixels_width,
        pixels_height,
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
    );

    println!("\r\nA pixel area of the size of a rectangle with width : {}px and height : {}px, will now be copied from the following screen location :", pixels_width, pixels_height);
    println!(
        "X : {}, Y : {}",
        screen_area_to_capture_upperleftcorner_x, screen_area_to_capture_upperleftcorner_y
    );
    println!(
        "and pasted to : X : {}, Y : {}",
        screen_destination_area_upperleftcorner_x, screen_destination_area_upperleftcorner_y
    );
}
