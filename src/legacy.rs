// Down here are the legacy functions

use crate::{bitblock_transfer, PixelValues};
pub use libc::c_void;
use windows::Win32::{
    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/
    Graphics::Gdi::{
        AlphaBlend, CreateBitmap, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC,
        DeleteObject, GetBitmapBits, GetDC, ReleaseDC, SelectObject, TransparentBlt, AC_SRC_ALPHA,
        AC_SRC_OVER, BLENDFUNCTION,
    },
};

/// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are retrieved row by row. Returns a Vec of specified type (u8/u32) ( e.g.: let vec_u8 :Vec<u8> = get_bytes(...) ).
pub fn get_bytes<T: PixelValues<T>>(
    area_width: u32,
    area_height: u32,
    src_ul_x: i32,
    src_ul_y: i32,
) -> Vec<T> {
    unsafe {
        // get a handle (H) to a device context (DC) for the client area,
        // in this case for the entire virtual screen (not just a monitor),
        // instead of a window (from hwnd value)
        // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
        let screen = GetDC(None);

        // Create a compatible bitmap of the requested pixel area (area_width x area_height px).
        // get a handle (H) of a memory device context (DC) from which capture data (pixels)
        let dst_screen = CreateCompatibleDC(screen);

        // requested pixels' area width and height to be captured
        let captured_hbmp = CreateCompatibleBitmap(screen, area_width as i32, area_height as i32);
        let hbmp_replace = SelectObject(dst_screen, captured_hbmp);

        // get the data of a given area from screen ad set it to captured_screen
        let dst_ul_x = 0;
        let dst_ul_y = 0;

        bitblock_transfer::bit_block_transfer(
            dst_screen,
            dst_ul_x,
            dst_ul_y,
            area_width,
            area_height,
            screen,
            src_ul_x,
            src_ul_y,
        );

        // create a Vec of u8 values (R,G,B(,A) values range : 0-255), with the size needed to represent the area of the screenshot to take
        //let mut vec :Vec<u8> = vec![0; area_width as usize * area_height as usize * 4];

        let mut vec = <T>::initialize_vec(area_width as usize, area_height as usize);
        //get_bitmap_bits(captured_hbmp, &mut vec);
        // add the captured area's pixels RGB values to the Vec
        GetBitmapBits(
            captured_hbmp,
            (area_width * area_height * 4) as i32,
            vec.as_mut_ptr() as *mut c_void,
        );

        //std::mem::forget(vec);
        ReleaseDC(None, dst_screen);
        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dst_screen, hbmp_replace);
        DeleteDC(dst_screen).unwrap();
        DeleteObject(captured_hbmp).unwrap();
        vec
    }
}

/// send Blue Green Red Alpha values to the pixels of a defined area of the screen
/// source_constant_alpha sets the Alpha value of every BGRA (so it sets the whole image's opacity , range : 0-255)
/// set source_constant_alpha to 255 in order to use per-pixel alpha values
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes<T: PixelValues<T> + Copy>(
    vec: &[T],
    area_width: u32,
    area_height: u32,
    dst_ul_x: i32,
    dst_ul_y: i32,
    source_constant_alpha: u8,
) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];

        let vec = &<T>::create_adjusted_vec(vec);

        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            area_width as i32,
            area_height as i32,
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
            Some(vec.as_ptr() as *mut c_void), //&vec as *const Vec<u8> as *mut c_void
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
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 1,
            // Set the SourceConstantAlpha value to 255 (opaque) when you only want to use per-pixel alpha values
            SourceConstantAlpha: source_constant_alpha,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };

        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-alphablend
        AlphaBlend(
            screen,
            dst_ul_x,
            dst_ul_y,
            area_width as i32,
            area_height as i32,
            dc_src,
            pixels_upperleftcorner_x,
            pixels_upperleftcorner_y,
            area_width as i32,
            area_height as i32,
            bf,
        )
        .unwrap();

        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src).unwrap();
        DeleteObject(hbmp_from_bytes).unwrap();
    }
}

/// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are sent row by row, the Alpha value in BlueGreenRedAlpha, that is used to define transparency,
/// will be ignored, as it will be max by default (255), so every pixel will have full opacity
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes_alpha_disabled<T>(
    vec: &[T],
    area_width: u32,
    area_height: u32,
    dst_ul_x: i32,
    dst_ul_y: i32,
) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];

        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            area_width as i32,
            area_height as i32,
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
            Some(vec.as_ptr() as *mut c_void), //&vec as *const Vec<u8> as *mut c_void
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
        bitblock_transfer::bit_block_transfer(
            screen,
            dst_ul_x,
            dst_ul_y,
            area_width,
            area_height,
            dc_src,
            pixels_upperleftcorner_x,
            pixels_upperleftcorner_y,
        );

        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src).unwrap();
        DeleteObject(hbmp_from_bytes).unwrap();
    }
}

/// send Blue Green Red Alpha (ignored) values to the pixels of a defined area of the screen
/// and make so that if a BGRA value to be sent to a pixel matches a specific (A is ignored)BGR (u32) value
/// that color will be sent as completely transparent
/// e.g., every time a white (B=255, G=255, R=255, A=any_u8_value) is to be sent to a pixel it must be sent as completely transparent, invisible, hidden
/// bgr_u32_to_hide must not contain A (e.g.: 0x00FF 8080, A must be zero for the hiding to work)
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes_alpha_disabled_hide_specific_bgr<T>(
    vec: &[T],
    area_width: u32,
    area_height: u32,
    dst_ul_x: i32,
    dst_ul_y: i32,
    hide_b: u8,
    hide_g: u8,
    hide_r: u8,
) {
    unsafe {
        //let mut vec :Vec<u8> = vec![0,0,255,255,0,0,255,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255,255,0,0,255];

        // create HBITMAP from a BGRA color pattern sequence array
        let hbmp_from_bytes = CreateBitmap(
            area_width as i32,
            area_height as i32,
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
            Some(vec.as_ptr() as *mut c_void), //&vec as *const Vec<u8> as *mut c_void
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

        let bgr_u32_to_hide = u32::from_ne_bytes([hide_r, hide_g, hide_b, 0]);

        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt
        TransparentBlt(
            screen,
            dst_ul_x,
            dst_ul_y,
            area_width as i32,
            area_height as i32,
            dc_src,
            pixels_upperleftcorner_x,
            pixels_upperleftcorner_y,
            area_width as i32,
            area_height as i32,
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
            bgr_u32_to_hide.to_owned(),
        )
        .unwrap();
        //std::mem::forget(vec);

        ReleaseDC(None, screen);
        // This function returns the previously selected object of the specified type.
        // An application should always replace a new object with the original,
        // default object after it has finished drawing with the new object.
        SelectObject(dc_src, hbmp_replace);
        DeleteDC(dc_src).unwrap();
        DeleteObject(hbmp_from_bytes).unwrap();
    }
}
