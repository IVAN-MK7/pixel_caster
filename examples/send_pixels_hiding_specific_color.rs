pub use pixel_caster::*;

fn main() {
    let mut bytes:Vec<u8> = Vec::new();
    //                         B  G    R    A
    //                               red
    bytes.extend_from_slice(&[ 0, 0, 255, 255 ]);
    //                           blue
    bytes.extend_from_slice(&[ 0, 255, 0, 255 ]);
    //                       green
    bytes.extend_from_slice(&[ 255, 0, 0, 255 ]);

    // same sequence but half the Alpha, which could be any u8 value
    // since all the colors will be sent with max opacity ( 255 Alpha, 0 transparency)
    // only the specific BGR specified in the function below will have a full transparency ( 0 Alpha, 0 opacity)
    bytes.extend_from_slice(&[ 0, 0, 255, 125 ]);
    bytes.extend_from_slice(&[ 0, 255, 0, 125 ]);
    bytes.extend_from_slice(&[ 255, 0, 0, 125 ]);

    // same but Alpha to 0
    bytes.extend_from_slice(&[ 0, 0, 255, 0 ]);
    bytes.extend_from_slice(&[ 0, 255, 0, 0 ]);
    bytes.extend_from_slice(&[ 255, 0, 0, 0 ]);

    // white, which is obtained by B=255 , G=255 , R=255
    bytes.extend_from_slice(&[ 255, 255, 255, 255 ]);
    bytes.extend_from_slice(&[ 255, 255, 255, 125 ]);
    bytes.extend_from_slice(&[ 255, 255, 255, 0 ]);

    bytes.extend_from_slice(&[ 255, 0, 255, 255 ]);
    bytes.extend_from_slice(&[ 255, 0, 255, 125 ]);
    bytes.extend_from_slice(&[ 255, 0, 255, 0 ]);
    bytes.extend_from_slice(&[ 255, 0, 100, 0 ]);
    
    let width = (bytes.len()/4) as i32;

    // send the Vec<u8> to the screen as an area 1pixel heigh, hide the whites (B=255 , G=255 , R=255)
    send_bytes_bgra_hide_specific_bgr(&mut bytes, &width, &1, &44, &600, bgra_to_u32_abgr(255, 255, 255, 0));
}