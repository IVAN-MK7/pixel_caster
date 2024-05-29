pub use pixel_caster::{bgra_management::SwitchBytes, *};

fn main() {
    let screen_area_to_capture_upperleftcorner_x = 80;
    let screen_area_to_capture_upperleftcorner_y = 2;
    let pixels_width = 4;
    let pixels_height = 1;

    // u8 Screen variant
    let mut screen_u8: Screen<u8> = Screen::new(80, 2, 4, 1);
    screen_u8.scan_area();

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

    // get the bytes from the pixels of the requested size from an absolute position on the screen into an already existing Vec<u8>
    //let mut vec_u8 = vec![0 as u8; pixels_width as usize * pixels_height as usize * 4];
    let mut vec_u8 = <u8>::initialize_vec(pixels_width, pixels_height);
    Screen::scan_area_custom(
        &mut vec_u8,
        screen_area_to_capture_upperleftcorner_x,
        screen_area_to_capture_upperleftcorner_y,
        pixels_width as u32,
        pixels_height as u32,
    )
    .unwrap();
    println!(
        "Vec<u8>: first value: {}, length: {}",
        vec_u8[0],
        vec_u8.len()
    );
    println!(
        "Vec<u8>: first pixel's values: B:{} G:{} R:{} A:{}",
        vec_u8[0], vec_u8[1], vec_u8[2], vec_u8[3]
    );
}
