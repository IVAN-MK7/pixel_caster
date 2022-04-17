pub use pixel_caster::*;

fn main (){
    let screen_destination_area_upperleftcorner_x = 30;
    let screen_destination_area_upperleftcorner_y = 90;
    let pixels_area_width = 4;
    let pixels_area_height = 4;

    /* Vec for testing : a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue*/
    let mut vec:Vec<u8> = Vec::with_capacity(4 * 4 * 4);
    // each byte (u8) has a range of 0-255, they are ordered by BGRA instead of RGBA
    //                      B G   R   A
    vec.extend_from_slice(&[0,0,255,255]); // 1 RED pixel
    vec.extend_from_slice(&[0,0,255,255]); // 1 RED pixel
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]); // 7 BLUE pixels
    vec.extend_from_slice(&[255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255]); // 7 BLUE pixels

    // send the bytes to the pixels of the requested size of an absolute position on the screen
    // for each pixel the Alpha value was set to 255, but it could be any value, since with the following function
    // every pixel will have Alpha automatically set to 255
    send_bytes_bgra_maxalpha(
        &mut vec,
        &pixels_area_width,
        &pixels_area_height,
        &screen_destination_area_upperleftcorner_x, 
        &screen_destination_area_upperleftcorner_y
    );

    println!("");
    println!("A Vec<u8> containing 64 bytes, representing a qube of 4 x 4 (16) pixels, where the first 2 are red, the other 14 blue, will now be sent to the screen at the location :");
    println!("X : {}, Y : {}", screen_destination_area_upperleftcorner_x, screen_destination_area_upperleftcorner_y);
    println!("The first pixel has the following RGBA values : R:{} G:{} B:{} A:{}", vec[2], vec[1], vec[0], vec[3]);
}