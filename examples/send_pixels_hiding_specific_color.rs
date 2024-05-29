pub use pixel_caster::{bgra_management::*, *};

fn main() {
    let screen_destination_area_upperleftcorner_x = 10;
    let screen_destination_area_upperleftcorner_y = 10;

    let mut bytes: Vec<u8> = Vec::new();
    //                        B  G   R    A
    //                              red
    bytes.extend_from_slice(&[0, 0, 255, 255]);
    //                          green
    bytes.extend_from_slice(&[0, 255, 0, 255]);
    //                        blue
    bytes.extend_from_slice(&[255, 0, 0, 255]);

    // same sequence but half the Alpha, which could be any u8 value
    // since in this example all the colors will be sent with max opacity ( 255 Alpha, 0 transparency)
    // only the BGR combination specified in the PixelsSendMode below will have a full transparency ( 0 Alpha, 0 opacity)
    bytes.extend_from_slice(&[0, 0, 255, 125]);
    bytes.extend_from_slice(&[0, 255, 0, 125]);
    bytes.extend_from_slice(&[255, 0, 0, 125]);

    // same but Alpha to 0
    bytes.extend_from_slice(&[0, 0, 255, 0]);
    bytes.extend_from_slice(&[0, 255, 0, 0]);
    bytes.extend_from_slice(&[255, 0, 0, 0]);

    // white, which is obtained by B=255 , G=255 , R=255
    bytes.extend_from_slice(&[255, 255, 255, 255]);
    bytes.extend_from_slice(&[255, 255, 255, 125]);
    bytes.extend_from_slice(&[255, 255, 255, 0]);

    bytes.extend_from_slice(&[255, 0, 255, 255]);
    bytes.extend_from_slice(&[255, 0, 255, 125]);
    bytes.extend_from_slice(&[255, 0, 255, 0]);
    bytes.extend_from_slice(&[255, 0, 100, 0]);

    // make an horizontal line of pixels from them
    let area_width = (bytes.len() / 4) as u32;
    let area_height = 1;

    println!("\r\nA Vec<u8> containing 64 bytes, representing a line of 16 x 1 pixels, will now be sent to the screen at the location :");
    println!(
        "X : {}, Y : {}",
        screen_destination_area_upperleftcorner_x, screen_destination_area_upperleftcorner_y
    );
    println!(
        "The first pixel has the following BGRA values : B:{} G:{} R:{} A:{}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    );

    // send the Vec<u8> to the screen as an area 1 pixel high, hide the whites (obtained with the B=255 , G=255 , R=255 combination)
    Screen::update_area_custom(
        &bytes,
        screen_destination_area_upperleftcorner_x,
        screen_destination_area_upperleftcorner_y,
        area_width,
        area_height,
        PixelsSendMode::AlphaDisabledHideBGR(255, 255, 255),
    );
}
