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
        GetBitmapBits
    }
};
pub use libc::c_void;

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

/// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are sent row by row.
pub fn send_bytes (vec :&mut Vec<u8>, req_width :&i32, req_height :&i32, dst_ul_x :&i32, dst_ul_y :&i32) {
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