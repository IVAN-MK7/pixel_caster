pub use pixel_caster::*;

fn main (){
    let screen_area_to_capture_upperleftcorner_x = 4;
    let screen_area_to_capture_upperleftcorner_y = 8;
    let pixels_width = 20;
    let pixels_height = 20;

    // get the bytes from the pixels of the requested size from an absolute position on the screen
    let vec = get_bytes(
        &pixels_width,
        &pixels_height,
        &screen_area_to_capture_upperleftcorner_x, 
        &screen_area_to_capture_upperleftcorner_y
    );

    println!("");
    println!("The 4 bytes of the first pixel analyzed from the selected location (X : {}, Y : {}), and area (width : {}, height : {}) of the screen", screen_area_to_capture_upperleftcorner_x, screen_area_to_capture_upperleftcorner_y, pixels_width, pixels_height);
    println!("represent the following RGBA values :");
    println!("R:{} G:{} B:{} A:{}", vec[2], vec[1], vec[0], vec[3]);
}