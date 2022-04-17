// https://github.com/microsoft/windows-rs
pub extern crate windows;
pub extern crate libc;
mod bitblock_transfer;

pub use windows::Win32::{
    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/
    Graphics::Gdi::{
        CreateBitmap,
        CreatedHDC,
        CreateCompatibleDC,
        CreateCompatibleBitmap,
        BitBlt,
        SelectObject,
        HDC,
        GetDC,
        SRCCOPY,
        ReleaseDC,
        DeleteDC,
        DeleteObject,
        GetBitmapBits,
        TransparentBlt,
        AlphaBlend,
        BLENDFUNCTION,
        AC_SRC_OVER,
        AC_SRC_ALPHA
    }
};
pub use libc::c_void;

pub fn get_vec_with_custom_capacity <T :'static> (req_width :&i32, req_height :&i32) ->  Result<Vec<T>, String> {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        return Ok(Vec::with_capacity((req_width * req_height) as usize));
    }
    else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u8>() {
        return Ok(Vec::with_capacity((req_width * req_height * 4) as usize));
    }
    return Err("provided Type (T in Vec<T>) not supported, only u8 and u32 are supported".to_string());
}


/// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are retrieved row by row. _test for auto setting a vector to either u8 or u32 bytes
/// CAN BE IMPLEMENTED WITH Traits, see below after the fn
pub fn get_bytes_t <T :'static> (req_width :&i32, req_height :&i32, src_ul_x :&i32, src_ul_y :&i32) -> Vec<T> {
    unsafe {
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        // Create a compatible bitmap of the requested pixel area (req_width x req_height px).
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dst_screen = CreateCompatibleDC(screen);

        // requested pixels' area width and height to be captured
        let captured_hbmp = CreateCompatibleBitmap(screen, req_width.to_owned(), req_height.to_owned());
        let hbmp_replace = SelectObject(dst_screen, captured_hbmp);
        
        // get the data of a given area from screen ad set it to captured_screen
        let dst_ul_x = 0;
        let dst_ul_y = 0;
        
        bitblock_transfer::to_screenshot(
            &dst_screen,
            &dst_ul_x,
            &dst_ul_y,
            &req_width,
            &req_height,
            &screen,
            &src_ul_x,
            &src_ul_y
        );


        // create a Vec of u8 values (R,G,B(,A) values range : 0-255), with the size needed to represent the area of the screenshot to take
        let mut vec :Vec<T> = match get_vec_with_custom_capacity(&req_width, &req_height) {
            Ok(v) => v,
            Err(err) => {
                panic!("{}",err);
            }
        };

        // add the captured area's pixels RGB values to the Vec
        // DOESENT WORK WITH VEC created with Vec::with_capacity(), only with vec![;]
        // with Vec<T> you cannot use vec![;], only Vec::with_capacity()
        GetBitmapBits(
            captured_hbmp,
            vec.capacity() as i32,
            vec.as_mut_ptr() as *mut c_void
        );

        //std::mem::forget(vec);
        ReleaseDC(None, dst_screen);
        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dst_screen, hbmp_replace);
        DeleteDC(dst_screen);
        DeleteObject(captured_hbmp);

        return vec;
    }
}

/*
trait Byte_retriever {
    fn give_five() -> Self;
}

impl Byte_retriever for Vec<u8> {
    fn give_five() -> Vec<u8> {
        return Vec::new();
    }
}

impl Byte_retriever for Vec<u32> {
    fn give_five() -> Vec<u32> {
        return Vec::new();
    }
}
*/
/*// usage
let y: Vec<u8> = Byte_retriever::give_five(); // returns 5
let z: Vec<u32> = Byte_retriever::give_five(); // return 5.0
*/

/// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are retrieved row by row.
pub fn get_bytes (req_width :&i32, req_height :&i32, src_ul_x :&i32, src_ul_y :&i32) -> Vec<u8> {
    unsafe {
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        // Create a compatible bitmap of the requested pixel area (req_width x req_height px).
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dst_screen = CreateCompatibleDC(screen);

        // requested pixels' area width and height to be captured
        let captured_hbmp = CreateCompatibleBitmap(screen, req_width.to_owned(), req_height.to_owned());
        let hbmp_replace = SelectObject(dst_screen, captured_hbmp);
        
        // get the data of a given area from screen ad set it to captured_screen
        let dst_ul_x = 0;
        let dst_ul_y = 0;
        
        bitblock_transfer::to_screenshot(
            &dst_screen,
            &dst_ul_x,
            &dst_ul_y,
            &req_width,
            &req_height,
            &screen,
            &src_ul_x,
            &src_ul_y
        );

        // create a Vec of u8 values (R,G,B(,A) values range : 0-255), with the size needed to represent the area of the screenshot to take
        let mut vec :Vec<u8> = vec![0; req_width.to_owned() as usize * req_height.to_owned() as usize * 4];

        //get_bitmap_bits(captured_hbmp, &mut vec);
        // add the captured area's pixels RGB values to the Vec
        GetBitmapBits(
            captured_hbmp,
            req_width * req_height * 4,
            vec.as_mut_ptr() as *mut c_void
        );

        //std::mem::forget(vec);
        ReleaseDC(None, dst_screen);
        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dst_screen, hbmp_replace);
        DeleteDC(dst_screen);
        DeleteObject(captured_hbmp);

        return vec;
    }
}

/// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are retrieved row by row. 32 bit version, a color is represented by [0xFFFF FFFF] instead of [0-255,0-255,0-255,0-255]
pub fn get_bytes_u32 (req_width :&i32, req_height :&i32, src_ul_x :&i32, src_ul_y :&i32) -> Vec<u32> {
    unsafe {
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        // Create a compatible bitmap of the requested pixel area (req_width x req_height px).
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dst_screen = CreateCompatibleDC(screen);

        // requested pixels' area width and height to be captured
        let captured_hbmp = CreateCompatibleBitmap(screen, req_width.to_owned(), req_height.to_owned());
        let hbmp_replace = SelectObject(dst_screen, captured_hbmp);
        
        // get the data of a given area from screen ad set it to captured_screen
        let dst_ul_x = 0;
        let dst_ul_y = 0;
        
        bitblock_transfer::to_screenshot(
            &dst_screen,
            &dst_ul_x,
            &dst_ul_y,
            &req_width,
            &req_height,
            &screen,
            &src_ul_x,
            &src_ul_y
        );

        // create a Vec of u32 values (R,G,B(,A) values range : 0-255), with the size needed to represent the area of the screenshot to take
        let mut vec :Vec<u32> = vec![0; req_width.to_owned() as usize * req_height.to_owned() as usize];

        //get_bitmap_bits(captured_hbmp, &mut vec);
        // add the captured area's pixels RGB values to the Vec
        GetBitmapBits(
            captured_hbmp,
            req_width * req_height,
            vec.as_mut_ptr() as *mut c_void
        );

        //std::mem::forget(vec);
        ReleaseDC(None, dst_screen);
        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dst_screen, hbmp_replace);
        DeleteDC(dst_screen);
        DeleteObject(captured_hbmp);

        return vec;
    }
}

/// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are sent row by row, the Alpha value in BlueGreenRedAlpha, that is used to define transparency,
/// will be ignored, as it will be max by default (255), so every pixel will have full opacity
pub fn send_bytes_bgra_maxalpha<T> (vec :&mut Vec<T>, req_width :&i32, req_height :&i32, dst_ul_x :&i32, dst_ul_y :&i32) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];
        
        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            req_width.to_owned(),
            req_height.to_owned(),
            1,
            // B        G        R        A
            // 0-255    0-255    0-255    0-255
            // 2^8 = 256 , so 8 bits are required to represent a color's range of values
            // so each one of the colors is represented by an unsigned 8 bit integer (u8)

            // a single pixel is formed by BGRA (Blue,Green,Red,Alpha)
            // the combination of their values gives the resulting color to a pixel
            // the range of each of their values : 0-255 (so : 0-255,0-255,0-255,0-255)
            // all 255 results in a black pixel, all 0 in a white pixel, 255,0,0,255 in a blue one
            // to represent values of range : 0-255 are necessary 8 bits ( 2^8 = 256 ), so the following bitcount must be 32
            // because 8+8+8+8 = 32, 32 bits are necessary to represent a pixel's combination of B,G,R,A
            32,
            // uses BGR format instead of RGB
            // https://stackoverflow.com/questions/31759582/assign-an-array-to-mut-c-void
            vec.as_mut_ptr() as *mut c_void
            //&vec as *const Vec<u8> as *mut c_void
        );
        
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dc_src = CreateCompatibleDC(None);
    
        let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
        
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        let pixels_upperleftcorner_x = 0;
        let pixels_upperleftcorner_y = 0;

        // bit-block transfer of the color data corresponding to an area of pixels
        // of the RGBA sequence it doesn't print the 4th value (A : alpha,opacity) so only RGB, the A won't be used
        bitblock_transfer::to_screen(
            &screen,
            &dst_ul_x,
            &dst_ul_y,
            &req_width, &req_height,
            &dc_src,
            &pixels_upperleftcorner_x,
            &pixels_upperleftcorner_y
        );
        
        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src);
        DeleteObject(hbmp_from_bytes);
    }
}

/// send Blue Green Red Alpha (ignored) values to the pixels of a defined area of the screen
/// and make so that if a BGRA value to be sent to a pixel matches a specific (A is ignored)BGR (u32) value
/// that color will be sent as completely transparent
/// e.g., every time a white (B=255, G=255, R=255, A=any_u8_value) is to be sent to a pixel it must be sent as completely transparent, invisible, hidden
pub fn send_bytes_bgra_hide_specific_bgr<T> (vec :&mut Vec<T>, req_width :&i32, req_height :&i32, dst_ul_x :&i32, dst_ul_y :&i32, abgr_u32_to_hide : u32) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];
        
        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            req_width.to_owned(),
            req_height.to_owned(),
            1,
            // B        G        R        A
            // 0-255    0-255    0-255    0-255
            // 2^8 = 256 , so 8 bits are required to represent a color's range of values
            // so each one of the colors is represented by an unsigned 8 bit integer (u8)

            // a single pixel is formed by BGRA (Blue,Green,Red,Alpha)
            // the combination of their values gives the resulting color to a pixel
            // the range of each of their values : 0-255 (so : 0-255,0-255,0-255,0-255)
            // all 255 results in a black pixel, all 0 in a white pixel, 255,0,0,255 in a blue one
            // to represent values of range : 0-255 are necessary 8 bits ( 2^8 = 256 ), so the following bitcount must be 32
            // because 8+8+8+8 = 32, 32 bits are necessary to represent a pixel's combination of B,G,R,A
            32,
            // uses BGR format instead of RGB
            // https://stackoverflow.com/questions/31759582/assign-an-array-to-mut-c-void
            vec.as_mut_ptr() as *mut c_void
            //&vec as *const Vec<u8> as *mut c_void
        );
        
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dc_src = CreateCompatibleDC(None);
    
        let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
        
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        let pixels_upperleftcorner_x = 0;
        let pixels_upperleftcorner_y = 0;


        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt
        TransparentBlt(
            screen.to_owned(),
            dst_ul_x.to_owned(),
            dst_ul_y.to_owned(),
            req_width.to_owned(),
            req_height.to_owned(),
            dc_src.to_owned(),
            pixels_upperleftcorner_x.to_owned(),
            pixels_upperleftcorner_y.to_owned(),
            req_width.to_owned(),
            req_height.to_owned(),

            // to hide (set transparency, Alpha to 0) the Blue ( obtained by value 255 of B value in B G R A) 
            // bytes.extend_from_slice(&[ 255, 0, 0, 255 ]);
            // represents B G R A, each has 8 bits, so :
            //             A         B         G         R
            // BIN 0000 0000 1111 1111 0000 0000 0000 0000
            //      A B  G R
            // HEX 00FF 0000
            // DEC  16711680
            // set crtransparent to 16711680

            
            // to hide (set transparency, Alpha to 0) the White ( obtained by value 255 of B,G,R combined, in B G R A) 
            // bytes.extend_from_slice(&[ 255, 255, 255, 255 ]);
            // represents B G R A, each has 8 bits, so :
            //             A         B         G         R
            // BIN 1111 1111 1111 1111 1111 1111 1111 1111
            //      A B  G R
            // HEX 00FF FFFF
            // because white is the sum of
            // full Blue
            // HEX 00FF 0000
            // DEC  16711680
            // full Green
            // HEX 0000 FF00
            // DEC     65280
            // full Red
            // HEX 0000 00FF
            // DEC       255
            // 16711680 + 65280 + 255 = 16777215
            // set crtransparent to 16777215

            // the A in BGRA will be ignored when matching the BGRA value between the one seeked and those present in the provided Vec<u8>
            // so the matching is done on the BGR values alone
            // use fn bgra_to_abgr_u32(b,g,r,a) to get the u32 value out of u8 bgra values
            abgr_u32_to_hide.to_owned()
          );
        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src);
        DeleteObject(hbmp_from_bytes);
    }
}

pub fn bgra_to_u32_abgr (b :u8, g :u8, r :u8, a :u8) -> u32 {
    return ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | ((r as u32) << 0);

    /*// example :
    let b :u8 = 250;
    let g :u8 = 122;
    let r :u8 = 35;
    let a :u8 = 125;
    
    // get the ABGR value as u32
    // HEX (4 values, each maxes out at FF)
    //    A B  G R
    // 0x7DFA 7A23
    //
    // BIN (4 values, each maxes out at 1111 1111, tot : 32 bits)
    //         A         B         G         R
    // 0111 1101 1111 1010 0111 1010 0010 0011
    //
    // DEC (4 values, each maxes out at 255 (max DEC value that a u8 can represent))
    //         A         B         G         R
    //       125       250       122        35

    // << shifts a number of bits to the left, >> to the right
    let abgr :u32 = ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | ((r as u32) << 0);

    // get the BGRA value as u32
    //    B G  R A
    // 0xFA7A 237D
    //         B         G         R         A
    // 1111 1010 0111 1010 0010 0011 0111 1101
    //         B         G         R         A
    //       250       122        35       125

    let bgra :u32 = ((b as u32) << 24) | ((g as u32) << 16) | ((r as u32) << 8) | ((a as u32) << 0);

    
    // get just B value
    let b_u32 = bgra >> 24;
    */
}

fn u32_abgr_to_u32_argb (mut pixels : &mut Vec<u32>) {
    let mut p : u32;
    let mut r : u32;
    let mut g : u32;
    let mut b : u32;
    let mut a : u32;
    for i in 0..pixels.len() {
        p = pixels[i];
        a = (p >> 24) & 0xFF; // get pixel bytes in ARGB order
        b = (p >> 16) & 0xFF;
        g = (p >> 8) & 0xFF;
        r = (p >> 0) & 0xFF;
        pixels[i] = (a << 24) | (r << 16) | (g << 8) | (b << 0);
    }
}

pub fn u8_bgra_to_u32_argb (b :u8, g :u8, r :u8, a :u8) -> u32 {
    return ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | ((b as u32) << 0);
}

pub fn vecu8_bgra_to_vecu32_argb (vec :&Vec<u8>) -> Vec<u32> {
    let mut vec32 :Vec<u32> = Vec::with_capacity(vec.len());

    let mut i = 0;
    for _ in 0..vec.len()/4 {
        vec32.extend_from_slice(&[(u8_bgra_to_u32_argb (vec[i],vec[i+1],vec[i+2],vec[i+3]))]);
        i += 4;
    }
    return vec32;
}

pub fn vecu32_argb_alpha_adjust (vec :&Vec<u32>) -> Vec<u32> {
    let mut vec_adjusted :Vec<u32> = Vec::with_capacity(vec.len());

    for i in 0..vec.len() {
        // let rgba_u32 = vec[i];
        // let just_alpha_u32 = vec[i] & 0xff000000;
        // let just_alpha_u32_0_255_range = vec[i] & 0xff000000 >> 24;

        // get Alpha Rred Green Blue values by excluding the others
        // for each make null the bytes that are not those representing the value we need
        // e.g., Alpha value : is represented by the 2 most left bytes, so with "ff" we keep them,
        // shift them enough position to have them at the most right so that they will be represented in a 0-255 range of values,
        // and assign to the variable "alpha"
        let alpha :u32 = (vec[i] & 0xff000000) >> 24;
        let red :u32 = (vec[i] & 0x00ff0000) >> 16;
        let green :u32 = (vec[i] & 0x0000ff00) >> 8;
        let blue :u32 = (vec[i] & 0x000000ff) >> 0;
        let diff :u32 = 255 - alpha;
        let mut red_adjusted :u32 = red;
        let mut green_adjusted :u32 = green;
        let mut blue_adjusted :u32 = blue;
        // in case the current ARGB value's Alpha is not at it's max (255 (u8), FF00 0000 (HEX), 4.278.190.080 (u32))
        // we need to adjust the values of the RGB bytes
        if diff > 0 {
            if red > 255 - diff {
                red_adjusted = 255 - diff;
            }
            if green > 255 - diff {
                green_adjusted = 255 - diff;
            }
            if blue > 255 - diff {
                blue_adjusted = 255 - diff;
            }
        }
        // get the RGBA values back to their u32 original positions
        vec_adjusted.extend_from_slice(&[(alpha << 24) + (red_adjusted << 16) + (green_adjusted << 8) + (blue_adjusted << 0)]);
    }
    return vec_adjusted;
}

pub fn vecu8_bgra_alpha_adjust (vec :&Vec<u8>) -> Vec<u8> {
    let mut vec_adjusted :Vec<u8> = Vec::with_capacity(vec.len());

    let mut i = 0;
    for _ in 0..vec.len()/4 {
        // get Alpha Rred Green Blue values
        let alpha = vec[i+3];
        let red = vec[i+2];
        let green = vec[i+1];
        let blue =  vec[i];
        let diff = 255 - alpha;
        let mut red_adjusted = red;
        let mut green_adjusted = green;
        let mut blue_adjusted = blue;
        // in case the current ARGB value's Alpha is not at it's max (255 (u8))
        // we need to adjust the values of the RGB bytes
        if diff > 0 {
            if red > 255 - diff {
                red_adjusted = 255 - diff;
            }
            if green > 255 - diff {
                green_adjusted = 255 - diff;
            }
            if blue > 255 - diff {
                blue_adjusted = 255 - diff;
            }
        }
        vec_adjusted.extend_from_slice(&[blue_adjusted, green_adjusted, red_adjusted, alpha]);

        i += 4;
    }
    return vec_adjusted;
}

/// send Blue Green Red Alpha values to the pixels of a defined area of the screen
/// source_constant_alpha sets the Alpha value of every BGRA (so it sets the whole image's opacity , range : 0-255)
/// set source_constant_alpha to 255 in order to use per-pixel alpha values
pub fn send_bytes_bgra<T> (vec :&mut Vec<T>, req_width :&i32, req_height :&i32, dst_ul_x :&i32, dst_ul_y :&i32, source_constant_alpha :u8) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];
        
        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            req_width.to_owned(),
            req_height.to_owned(),
            1,
            // B        G        R        A
            // 0-255    0-255    0-255    0-255
            // 2^8 = 256 , so 8 bits are required to represent a color's range of values
            // so each one of the colors is represented by an unsigned 8 bit integer (u8)

            // a single pixel is formed by BGRA (Blue,Green,Red,Alpha)
            // the combination of their values gives the resulting color to a pixel
            // the range of each of their values : 0-255 (so : 0-255,0-255,0-255,0-255)
            // all 255 results in a black pixel, all 0 in a white pixel, 255,0,0,255 in a blue one
            // to represent values of range : 0-255 are necessary 8 bits ( 2^8 = 256 ), so the following bitcount must be 32
            // because 8+8+8+8 = 32, 32 bits are necessary to represent a pixel's combination of B,G,R,A
            32,
            // uses BGRA format instead of RGBA
            // https://stackoverflow.com/questions/31759582/assign-an-array-to-mut-c-void
            vec.as_mut_ptr() as *mut c_void
            //&vec as *const Vec<u8> as *mut c_void
        );
        
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dc_src = CreateCompatibleDC(None);
    
        let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
        
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        
        // get a handle (H) of a memory device context (DC) to which send data (pixels/BGRA colors)
        let screen = GetDC(None);

        let pixels_upperleftcorner_x = 0;
        let pixels_upperleftcorner_y = 0;

        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-blendfunction
        let bf = BLENDFUNCTION {
            BlendOp : AC_SRC_OVER as u8,
            BlendFlags : 1,
            // Set the SourceConstantAlpha value to 255 (opaque) when you only want to use per-pixel alpha values
            SourceConstantAlpha : source_constant_alpha,
            AlphaFormat : AC_SRC_ALPHA as u8
        };

        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-alphablend
        AlphaBlend(
            screen.to_owned(),
            dst_ul_x.to_owned(),
            dst_ul_y.to_owned(),
            req_width.to_owned(),
            req_height.to_owned(),
            dc_src.to_owned(),
            pixels_upperleftcorner_x.to_owned(),
            pixels_upperleftcorner_y.to_owned(),
            req_width.to_owned(),
            req_height.to_owned(),
            bf
          );
        
        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src);
        DeleteObject(hbmp_from_bytes);
    }
}

/// Copies the pixels from a given area of the screen and pastes them into another given area of the screen.
pub fn copy_and_paste_pixels(req_width :&i32, req_height :&i32, src_ul_x :&i32, src_ul_y :&i32, dst_ul_x :&i32, dst_ul_y :&i32) {
    unsafe {
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        // Create a compatible bitmap of the requested pixel area (req_width x req_height px).
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let captured_screen = CreateCompatibleDC(screen);

        // requested pixels' area width and height to be captured
        let captured_hbmp = CreateCompatibleBitmap(screen, req_width.to_owned(), req_height.to_owned());
        
        let hbmp_replace = SelectObject(captured_screen, captured_hbmp);
        
        // get the data of a given area from screen ad set it to captured_screen
        let captured_screen_upperleftcorner_x = 0;
        let captured_screen_upperleftcorner_y = 0;
        
        bitblock_transfer::to_screenshot(
            &captured_screen,
            &captured_screen_upperleftcorner_x,
            &captured_screen_upperleftcorner_y,
            &req_width,
            &req_height,
            &screen,
            &src_ul_x,
            &src_ul_y
        );

        // print to screen
        // source and destination pixel area set as the same
        let pixels_to_print_width = req_width;
        let pixels_to_print_height = req_height;
        let pixels_upperleftcorner_x = 0;
        let pixels_upperleftcorner_y = 0;
        
        bitblock_transfer::to_screen(
            &screen,
            &dst_ul_x,
            &dst_ul_y,
            &pixels_to_print_width,
            &pixels_to_print_height,
            &captured_screen,
            &pixels_upperleftcorner_x,
            &pixels_upperleftcorner_y
        );

        ReleaseDC(None, captured_screen);
        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(captured_screen, hbmp_replace);
        DeleteDC(captured_screen);
        DeleteObject(captured_hbmp);
    }
}

// based on
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen