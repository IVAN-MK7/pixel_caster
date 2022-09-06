// https://github.com/microsoft/windows-rs
extern crate windows;
extern crate libc;
#[macro_use]
pub mod macros;


mod bitblock_transfer;
pub mod bgra_management;

#[cfg(feature = "pixels_string")]
pub mod pixels_string;

use bgra_management::u32_bytes_oredered_indexes_and_fullvalues;
use pixels_string::PixelsCollection;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::{
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

/// Stuff used to work with the winapi
pub struct WindowsApiScreen {
    /// Gets a handle (H) to a device context (DC) for the client area,
    /// in this case for the entire virtual screen (not just a monitor),
    /// instead of a window (from hwnd value)
    /// this is the handle (H) of a memory device context (DC) to which send data (BGRA colors)
    /// Used either as a handle (H) of a memory device context (DC) to/from which send/capture data (BGRA colors)
    screen: HDC,
    /// Create a compatible bitmap of the requested pixel area (area_width x area_height px).
    /// Used either as a handle (H) of a memory device context (DC) from/to which capture/send data (BGRA colors)
    dc_screen: CreatedHDC,
    /// requested pixels' area width and height to be captured
    captured_hbmp: HBITMAP,
    /// Determines if the values are to keep after use or not
    is_static: bool
}

/// Contains the values needed to locate the area of the screen to work with
pub struct ScreenArea {
    /// X dimension (horizontal) position of the upper left corner of the rectangle that delimits the needed screen area
    upperleftcorner_x: i32,
    /// Y dimension (vertical) position of the upper left corner of the rectangle that delimits the needed screen area
    upperleftcorner_y: i32,
    /// Width of the rectangle that delimits the needed screen area
    width: u32,
    /// Height of the rectangle that delimits the needed screen area
    height: u32
}

/// Screen is used to get/send color bytes from/to the screen in a straightforward way
pub struct Screen<T: PixelValues<T> + Copy> {
    /// PixelsCollection containing color bytes data and info
    pixels: PixelsCollection<T>,
    screen_area: ScreenArea,
    win_api_screen: WindowsApiScreen,
    pixels_send_mode: PixelsSendMode
}

/// Defines how the BGRA bytes representing pixels' colors sent to the screen must be treated
#[derive(Clone,Copy)]
pub enum PixelsSendMode {
    /// Pixels will be sent with the Alpha channel enabled
    AlphaEnabled,
    /// Pixels will be sent with the Alpha channel Disabled (each BGRA's Alpha value will be sent as 255 (full opacity, no transparency))
    AlphaDisabled,
    /// Pixels will be sent with the Alpha channel Disabled (each BGRA's Alpha value will be sent as 255 (full opacity, no transparency)),
    /// and the color resulting from the given BGR u8 values combination will be sent as fully transparent (no opacity, max transparency)
    AlphaDisabledHideBGR(u8,u8,u8),
    /// Pixels will be sent with the provided u8 Alpha value, instead of their own
    CustomAlpha(u8),
}

impl<T: PixelValues<T> + Copy> Screen<T> {

    /// Initializes a new Screen instance
    pub fn new(screen_area_upperleftcorner_x: i32, screen_area_upperleftcorner_y: i32, area_width: u32, area_height: u32) -> Screen<T> {
        
        let mut bytes = Vec::<T>::with_capacity((area_width * area_height * <T>::units_per_pixel() as u32) as usize);
        unsafe { bytes.set_len(bytes.capacity()); }
        
        Screen {
            pixels: PixelsCollection::<T>::create(area_width as usize, area_height as usize, bytes).unwrap(),
            screen_area: ScreenArea { upperleftcorner_x: screen_area_upperleftcorner_x, upperleftcorner_y: screen_area_upperleftcorner_y, width: area_width, height: area_height},
            win_api_screen : Self::gen_win_api_screen(area_width, area_height, true),
            pixels_send_mode: PixelsSendMode::AlphaEnabled
        }
    }

    /// Prepares the stuff needed to make the Windows API to manage the screen's pixel's data
    fn gen_win_api_screen(area_width: u32, area_height: u32, is_static: bool) -> WindowsApiScreen {
        unsafe {
            let screen = GetDC(None);
            WindowsApiScreen { screen, dc_screen: CreateCompatibleDC(screen), captured_hbmp: CreateCompatibleBitmap(screen, area_width as i32, area_height as i32), is_static}
        }
    }
    
    /// Returns a reference to its PixelsCollection's bytes Vec
    pub fn get_bytes(&self) -> &Vec<T> {
        &self.pixels.bytes
    }
    
    /// Returns a reference to its PixelsCollection
    pub fn get_pixels_collection(&self) -> &PixelsCollection<T> {
        &self.pixels
    }

    /// Updates its PixelsCollection's bytes with the BGRA bytes of the Screen's set pixels area
    pub fn scan_area(&mut self) {
        Self::get_bytes_from_screen(self.pixels.bytes.as_mut_ptr() as *mut c_void,
            self.screen_area.upperleftcorner_x as i32, self.screen_area.upperleftcorner_y as i32,
            self.screen_area.width as u32, self.screen_area.height as u32,
            &self.win_api_screen
        )
    }
    
    /// Puts the BGRA bytes of the Screen's set pixels area into the provided Vec, which must already have the necessary length to host the values
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// let screen_area_upperleftcorner_x = 100;
    /// let screen_area_upperleftcorner_y = 100;
    /// let area_width = 200;
    /// let area_height = 200;
    /// 
    /// // the provided Vec must already have the necessary length to host the values.
    /// // Which can be obtained either by manually setting its length :
    /// let mut vec_u8_size_set = Vec::<u8>::with_capacity((area_width * area_height * <u8>::units_per_pixel() as u32) as usize);
    /// unsafe { vec_u8_size_set.set_len(vec_u8_size_set.capacity()); }
    /// 
    /// // or by pre populating it with the needed number of values to reach the needed size :
    /// let mut vec_u8_pre_populated = vec![0 as u8; (area_width * area_height * <u8>::units_per_pixel() as u32) as usize];
    /// 
    /// let screen = Screen::<u8>::new(screen_area_upperleftcorner_x, screen_area_upperleftcorner_y, area_width, area_height);
    /// screen.scan_area_onto_vec(&mut vec_u8_pre_populated).unwrap();
    /// ```
    pub fn scan_area_onto_vec(&self, vec: &mut Vec<T>) -> Result<(), String> {
        //if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        if vec.len() != (self.screen_area.width * self.screen_area.height * <T>::units_per_pixel() as u32) as usize {
            return Err("Provided Vec has not the correct length".to_string());
        }
        return Ok(Self::get_bytes_from_screen(vec.as_mut_ptr() as *mut c_void, self.screen_area.upperleftcorner_x, self.screen_area.upperleftcorner_y, self.screen_area.width, self.screen_area.height, &self.win_api_screen));
    }
    
    /// Puts the BGRA bytes of a given area of the screen into the provided Vec, which must already have the necessary length to host the values
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// let screen_area_to_capture_upperleftcorner_x = 80;
    /// let screen_area_to_capture_upperleftcorner_y = 2;
    /// let area_width = 4;
    /// let area_height = 1;
    /// 
    /// // the provided Vec must already have the necessary length to host the values.
    /// // Which can be obtained either by manually setting its length :
    /// let mut vec_u8_size_set = Vec::<u8>::with_capacity((area_width * area_height * <u8>::units_per_pixel() as u32) as usize);
    /// unsafe { vec_u8_size_set.set_len(vec_u8_size_set.capacity()); }
    /// 
    /// // or by pre populating it with the needed number of values to reach the needed size :
    /// let mut vec_u8_pre_populated = vec![0 as u8; (area_width * area_height * <u8>::units_per_pixel() as u32) as usize];
    /// Screen::get_bytes_onto_vec(
    ///     &mut vec_u8_pre_populated,
    ///     screen_area_to_capture_upperleftcorner_x, 
    ///     screen_area_to_capture_upperleftcorner_y,
    ///     area_width,
    ///     area_height
    /// ).unwrap();
    /// ```
    pub fn scan_area_custom(vec: &mut Vec<T>, src_ul_x: i32, src_ul_y: i32, area_width: u32, area_height: u32) -> Result<(), String> {
        //if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        if vec.len() != (area_width * area_height * <T>::units_per_pixel() as u32) as usize {
            return Err("Provided Vec has not the correct length".to_string());
        }
        return Ok(Self::get_bytes_from_screen(vec.as_mut_ptr() as *mut c_void, src_ul_x, src_ul_y, area_width, area_height, &Self::gen_win_api_screen(area_width, area_height, false)));
    }
    
    /// Sends its PixelsCollection's bytes to the Screen's set pixels area
    pub fn update_area(&mut self) {
        Self::pixels_send_mode_matcher(&self.pixels.bytes,
            self.screen_area.upperleftcorner_x as i32, self.screen_area.upperleftcorner_y as i32,
            self.screen_area.width as u32, self.screen_area.height as u32, &self.win_api_screen, self.pixels_send_mode)
    }
    
    /// Sends the provided Vec's values to the Screen's set pixels area
    pub fn update_area_from_vec(&mut self, vec: &Vec<T>) {
        Self::pixels_send_mode_matcher(vec,
            self.screen_area.upperleftcorner_x as i32, self.screen_area.upperleftcorner_y as i32,
            self.screen_area.width as u32, self.screen_area.height as u32, &self.win_api_screen, self.pixels_send_mode)
    }

    /// Sends a provided Vec's values to the provided screen area with the given PixelsSendMode, without creating a Screen instance
    pub fn update_area_custom(vec: &Vec<T>, screen_area_upperleftcorner_x: i32, screen_area_upperleftcorner_y: i32, area_width: u32, area_height: u32, pixels_send_mode: PixelsSendMode) {
        Self::pixels_send_mode_matcher(vec,
            screen_area_upperleftcorner_x, screen_area_upperleftcorner_y,
            area_width, area_height, &Self::gen_win_api_screen(area_width, area_height, false), pixels_send_mode)
    }
    
    fn pixels_send_mode_matcher(vec: &Vec<T>, screen_area_upperleftcorner_x: i32, screen_area_upperleftcorner_y: i32, area_width: u32, area_height: u32, win_api_screen: &WindowsApiScreen, pixels_send_mode: PixelsSendMode) {
        match pixels_send_mode {
            PixelsSendMode::AlphaEnabled => Self::send_bytes_to_screen(vec,
                screen_area_upperleftcorner_x as i32, screen_area_upperleftcorner_y as i32,
                area_width, area_height,
                255, win_api_screen
            ),
            PixelsSendMode::AlphaDisabled => Self::send_bytes_to_screen_alpha_disabled(vec,
                screen_area_upperleftcorner_x as i32, screen_area_upperleftcorner_y as i32,
                area_width, area_height, win_api_screen
            ),
            PixelsSendMode::AlphaDisabledHideBGR(b,g,r) => Self::send_bytes_to_screen_alpha_disabled_hide_specific_bgr(vec,
                screen_area_upperleftcorner_x as i32, screen_area_upperleftcorner_y as i32,
                area_width, area_height, win_api_screen,b,g,r
            ),
            PixelsSendMode::CustomAlpha(custom_alpha) => {
                if custom_alpha == 255 {
                    Self::send_bytes_to_screen_alpha_disabled(vec,
                        screen_area_upperleftcorner_x as i32, screen_area_upperleftcorner_y as i32,
                        area_width, area_height, win_api_screen
                    );
                }
                else {
                    Self::send_bytes_to_screen(vec,
                        screen_area_upperleftcorner_x as i32, screen_area_upperleftcorner_y as i32,
                        area_width, area_height, custom_alpha, win_api_screen
                    );
                }
            }
        }
    }

    /// send Blue Green Red Alpha values to the pixels of a defined area of the screen
    /// source_constant_alpha sets the Alpha value of every BGRA (so it sets the whole image's opacity , range : 0-255)
    /// set source_constant_alpha to 255 in order to use per-pixel alpha values
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen(vec :&Vec<T>, dst_ul_x :i32, dst_ul_y :i32, area_width :u32, area_height :u32, source_constant_alpha :u8, win_api_screen: &WindowsApiScreen) {
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
                vec.as_ptr() as *mut c_void
                //&vec as *const Vec<u8> as *mut c_void
            );
            let hbmp_replace = SelectObject(win_api_screen.dc_screen, hbmp_from_bytes);

            // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-blendfunction
            let bf = BLENDFUNCTION {
                BlendOp : AC_SRC_OVER as u8,
                BlendFlags : 1,
                // Set the SourceConstantAlpha value to 255 (opaque) when you only want to use per-pixel alpha values
                SourceConstantAlpha : source_constant_alpha,
                // This flag is set when the bitmap has an Alpha channel (that is, per-pixel alpha).
                // The Windows API use premultiplied alpha, which means that the Red, Green and Blue channel values must be premultiplied with the Alpha channel value.
                // For example, if the alpha channel value is x, the Red, Green and Blue channels must be multiplied by x and divided by 0xff (255) prior to the call.
                AlphaFormat : AC_SRC_ALPHA as u8
            };
    
            // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-alphablend
            AlphaBlend(
                //screen,
                // the entire virtual screen (not just a monitor)
                win_api_screen.screen,
                dst_ul_x,
                dst_ul_y,
                area_width as i32,
                area_height as i32,
                //dc_src,
                win_api_screen.dc_screen,
                0,
                0,
                area_width as i32,
                area_height as i32,
                bf
            );
    
            // If the win_api_screen were made for a single run delete them. Must delete the created elements, otherwise after many calls the api will stop working for the whole duration of this .exe process
            if !win_api_screen.is_static {
                ReleaseDC(None, win_api_screen.screen);
                // This function returns the previously selected object of the specified type.
                // An application should always replace a new object with the original,
                // default object after it has finished drawing with the new object.
                SelectObject(win_api_screen.dc_screen, hbmp_replace);
                DeleteDC(win_api_screen.dc_screen);
                DeleteObject(win_api_screen.captured_hbmp);
                DeleteObject(hbmp_from_bytes);
            }
        }
    }
    
    /// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
    /// The bytes are sent row by row, the Alpha value in BlueGreenRedAlpha, that is used to define transparency,
    /// will be ignored, as it will be max by default (255), so every pixel will have full opacity
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen_alpha_disabled(vec :&Vec<T>, dst_ul_x :i32, dst_ul_y :i32, area_width :u32, area_height :u32, win_api_screen: &WindowsApiScreen) {
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
                vec.as_ptr() as *mut c_void
                //&vec as *const Vec<u8> as *mut c_void
            );
            
            // get a handle (H) of a memory device context (DC) from which capture data (pixels)
            //let dc_src = CreateCompatibleDC(None);
        
            //let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
            let hbmp_replace = SelectObject(win_api_screen.dc_screen, hbmp_from_bytes);
            
            
            // get a handle (H) to a device context (DC) for the client area,
            // in this case for the entire virtual screen (not just a monitor),
            // instead of a window (from hwnd value)
            
            // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
            //let screen = GetDC(None);

            let pixels_upperleftcorner_x = 0;
            let pixels_upperleftcorner_y = 0;

            // bit-block transfer of the color data corresponding to an area of pixels
            // of the RGBA sequence it doesn't print the 4th value (A : alpha,opacity) so only RGB, the A won't be used
            bitblock_transfer::bit_block_transfer(
                //screen,
                win_api_screen.screen,
                dst_ul_x,
                dst_ul_y,
                area_width, area_height,
                //dc_src,
                win_api_screen.dc_screen,
                pixels_upperleftcorner_x,
                pixels_upperleftcorner_y
            );
            
            //std::mem::forget(vec);

            // If the win_api_screen were made for a single run delete them
            if !win_api_screen.is_static {
                ReleaseDC(None, win_api_screen.screen);
                // This function returns the previously selected object of the specified type.
                // An application should always replace a new object with the original,
                // default object after it has finished drawing with the new object.
                SelectObject(win_api_screen.dc_screen, hbmp_replace);
                DeleteDC(win_api_screen.dc_screen);
                DeleteObject(hbmp_from_bytes);
            }
        }
    }

    /// send Blue Green Red Alpha (ignored) values to the pixels of a defined area of the screen
    /// and make so that if a BGRA value to be sent to a pixel matches a specific (A is ignored)BGR (u32) value
    /// that color will be sent as completely transparent
    /// e.g., every time a white (B=255, G=255, R=255, A=any_u8_value) is to be sent to a pixel it must be sent as completely transparent, invisible, hidden
    /// bgr_u32_to_hide must not contain A (e.g.: 0x00FF 8080, A must be zero for the hiding to work)
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen_alpha_disabled_hide_specific_bgr(vec :&Vec<T>, dst_ul_x :i32, dst_ul_y :i32, area_width :u32, area_height :u32, win_api_screen: &WindowsApiScreen, hide_b :u8, hide_g :u8, hide_r :u8) {
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
                vec.as_ptr() as *mut c_void
                //&vec as *const Vec<u8> as *mut c_void
            );
            
            // get a handle (H) of a memory device context (DC) from which capture data (pixels)
            //let dc_src = CreateCompatibleDC(None);
            
            //let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
            let hbmp_replace = SelectObject(win_api_screen.dc_screen, hbmp_from_bytes);
            
            // get a handle (H) to a device context (DC) for the client area,
            // in this case for the entire virtual screen (not just a monitor),
            // instead of a window (from hwnd value)
            
            // get a handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
            //let screen = GetDC(None);
    
            let pixels_upperleftcorner_x = 0;
            let pixels_upperleftcorner_y = 0;
            
            let bgr_u32_to_hide = u32::from_ne_bytes([hide_r, hide_g, hide_b, 0]);
    
            // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt
            TransparentBlt(
                //screen,
                win_api_screen.screen,
                dst_ul_x,
                dst_ul_y,
                area_width as i32,
                area_height as i32,
                //dc_src,
                win_api_screen.dc_screen,
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
                bgr_u32_to_hide.to_owned()
              );
            //std::mem::forget(vec);
    
            // If the win_api_screen were made for a single run delete them
            if !win_api_screen.is_static {
                ReleaseDC(None, win_api_screen.screen);
                // This function returns the previously selected object of the specified type.
                // An application should always replace a new object with the original,
                // default object after it has finished drawing with the new object.
                SelectObject(win_api_screen.dc_screen, hbmp_replace);
                DeleteDC(win_api_screen.dc_screen);
                DeleteObject(hbmp_from_bytes);
            }
        }
    }
    

    /// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
    /// The bytes are retrieved row by row. Return a Vec of specified type ( e.g.: let vec_u8 :Vec<u8> = Vec::get_bytes(...) ).
    fn get_bytes_from_screen(self_vec :*mut c_void, src_ul_x :i32, src_ul_y :i32, area_width :u32, area_height :u32, win_api_screen: &WindowsApiScreen) {
        unsafe {
            let hbmp_replace = SelectObject(win_api_screen.dc_screen, win_api_screen.captured_hbmp);

            bitblock_transfer::bit_block_transfer(
                win_api_screen.dc_screen,
                0,
                0,
                area_width,
                area_height,
                win_api_screen.screen,
                src_ul_x,
                src_ul_y
            );
    
            // create a Vec of u32 values (B,G,R,A values range : 0-255), with the size needed to represent the area of the screenshot to take
            // let mut vec :Vec<u32> = vec![0; area_width.to_owned() as usize * area_height.to_owned() as usize];
            // let mut vec :Vec<u8> = vec![0; area_width.to_owned() as usize * area_height.to_owned() as usize * 4];
            
            // add the captured area's pixels BGRA values to the Vec
            // DOESENT WORK WITH VEC created with Vec::with_capacity(), only with already populated, e.g.: with vec![;]
            // to make it work with Vec<T> created with Vec::with_capacity(area_width*area_height*4) must then call vec.set_len(area_width*area_height*4) , which is unsafe
            GetBitmapBits(
                //captured_hbmp,
                win_api_screen.captured_hbmp,
                (area_width * area_height * <T>::units_per_pixel() as u32) as i32,
                self_vec
                //*vec.as_mut_ptr() as *mut c_void
            );

            // If the win_api_screen were made for a single run delete them. Must delete the created elements, otherwise after many calls the api will stop working for the whole duration of this .exe process
            if !win_api_screen.is_static {
                ReleaseDC(None, win_api_screen.dc_screen);
                ReleaseDC(None, win_api_screen.screen);
                // This function returns the previously selected object of the specified type.
                // An application should always replace a new object with the original,
                // default object after it has finished drawing with the new object.
                SelectObject(win_api_screen.dc_screen, hbmp_replace);
                DeleteDC(win_api_screen.dc_screen);
                DeleteObject(win_api_screen.captured_hbmp);
            }

        }
    }
    
    /// Copies the pixels from a given area of the screen and pastes them onto another given area of the screen.
    pub fn copy_and_paste_pixels(src_ulc_x :i32, src_ulc_y :i32, area_width :u32, area_height :u32, dst_ulc_x :i32, dst_ulc_y :i32) {
        unsafe {
            // get a handle (H) to a device context (DC) for the client area,
            // in this case for the entire virtual screen (not just a monitor),
            // instead of a window (from hwnd value)
            // this is the handle (H) of a memory device context (DC) to which send data (pixels/RGB(A) colors)
            let screen = GetDC(None);
    
            // Create a compatible bitmap of the requested pixel area (area_width x area_height px).
            // get a handle (H) of a memory device context (DC) from which capture data (pixels)
            let captured_screen = CreateCompatibleDC(screen);
    
            // requested pixels' area width and height to be captured
            let captured_hbmp = CreateCompatibleBitmap(screen, area_width as i32, area_height as i32);
            
            let hbmp_replace = SelectObject(captured_screen, captured_hbmp);
            
            // get the data of a given area from screen ad set it to captured_screen
            let captured_screen_upperleftcorner_x = 0;
            let captured_screen_upperleftcorner_y = 0;
            
            bitblock_transfer::bit_block_transfer(
                captured_screen,
                captured_screen_upperleftcorner_x,
                captured_screen_upperleftcorner_y,
                area_width,
                area_height,
                screen,
                src_ulc_x,
                src_ulc_y
            );
    
            // print to screen
            // source and destination pixel area set as the same
            let pixels_to_print_width = area_width;
            let pixels_to_print_height = area_height;
            let pixels_upperleftcorner_x = 0;
            let pixels_upperleftcorner_y = 0;
            
            bitblock_transfer::bit_block_transfer(
                screen,
                dst_ulc_x,
                dst_ulc_y,
                pixels_to_print_width,
                pixels_to_print_height,
                captured_screen,
                pixels_upperleftcorner_x,
                pixels_upperleftcorner_y
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

}

/// Permits only to Vec\<u8> and Vec\<u32> to be used
pub trait PixelValues<T> {
    /// Adjusts pixels' Alpha values for when they must be sent with Alpha channel enabled,
    /// this will make so that each pixel's transparency is adjusted to the value the Windows API needs to display it with the wanted transparency.
    /// The Windows API use premultiplied alpha, which means that the Red, Green and Blue channel values must be premultiplied with the Alpha channel value.
    /// For example, if the alpha channel value is x, the Red, Green and Blue channels must be multiplied by x and divided by 0xff (255) prior to the call.
    fn create_adjusted_vec(vec :&Vec<T>) -> Vec<T>;

    /// Creates a Vec of the given type and sets its capacity and length needed to contain the pixels' color values of the of the given area (u8 : area_width * area_height * 4, u32 : area_width * area_height)
    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<T>;

    /// How many units of this type of value are necessary to represent a single pixel's color (u8 : 4 (1Blue,1Red,1Green,1Alpha), u32 : 1 (0xAARRGGBB))
    fn units_per_pixel() -> u8;

    /// Returns .units_per_pixel() for the type specified at &self
    fn get_units_per_pixel(&self) -> u8;
}
impl PixelValues<u8> for u8 {
    fn create_adjusted_vec(vec :&Vec<u8>) -> Vec<u8> {
        let mut vec_adjusted :Vec<u8> = Vec::with_capacity(vec.len());
    
        let mut i = 0;
        for _ in 0..vec.len()/4 {
            // get Alpha Red Green Blue values
            let blue =  vec[i];
            let green = vec[i+1];
            let red = vec[i+2];
            let alpha = vec[i+3];
            let diff = 255 - alpha;
            let mut blue_adjusted = blue;
            let mut green_adjusted = green;
            let mut red_adjusted = red;
            // in case the current BGRA value's Alpha is not at it's max (255 (u8))
            // we need to adjust the values of the BGR bytes
            if diff > 0 {
                if blue > 255 - diff {
                    blue_adjusted = 255 - diff;
                }
                if green > 255 - diff {
                    green_adjusted = 255 - diff;
                }
                if red > 255 - diff {
                    red_adjusted = 255 - diff;
                }
            }
            vec_adjusted.extend_from_slice(&[blue_adjusted, green_adjusted, red_adjusted, alpha]);
    
            i += 4;
        }
        return vec_adjusted;
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u8> {
        let mut vec_u8_size_set = Vec::<u8>::with_capacity((area_width * area_height * <u8>::units_per_pixel() as usize));
        unsafe { vec_u8_size_set.set_len(vec_u8_size_set.capacity()); }
        return vec_u8_size_set
    }

    fn units_per_pixel() -> u8 { 4 }
    fn get_units_per_pixel(&self) -> u8 { 4 }
}
impl PixelValues<u32> for u32 {
    fn create_adjusted_vec(vec :&Vec<u32>) -> Vec<u32> {
        let mut vec_adjusted :Vec<u32> = Vec::with_capacity(vec.len());
        
        let (ordered_indexes, fullvalues)= u32_bytes_oredered_indexes_and_fullvalues();
        let v1_index = ordered_indexes[0];
        let v2_index = ordered_indexes[1];
        let v3_index = ordered_indexes[2];
        let v4_index = ordered_indexes[3];
        let v1_full_val = fullvalues[0];
        let v2_full_val = fullvalues[1];
        let v3_full_val = fullvalues[2];
        let v4_full_val = fullvalues[3];
    
        for i in 0..vec.len() {
    
            // get Blue Green Red Alpha values by excluding the others
            // for each make null the bytes that are not those representing the value we need
            // e.g., Alpha value : is the 4th in BGRA (in a CPU with little endianness is represented by the 2 most left bytes), so with "0xFF00_0000" we keep them,
            // shift them enough position to have them at the most right so that they will be represented in a 0-255 range of values,
            // and assign to the variable "alpha"
            let blue :u32 = (vec[i] & v1_full_val) >> v1_index;
            let green :u32 = (vec[i] & v2_full_val) >> v2_index;
            let red :u32 = (vec[i] & v3_full_val) >> v3_index;
            let alpha :u32 = (vec[i] & v4_full_val) >> v4_index;
            let diff :u32 = 255 - alpha;
            let mut red_adjusted :u32 = red;
            let mut green_adjusted :u32 = green;
            let mut blue_adjusted :u32 = blue;
            // in case the current BGRA value's Alpha is not at it's max (255 (u8), FF00 0000 (HEX), 4.278.190.080 (u32))
            // we need to adjust the values of the BGR bytes
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
            // set the BGRA values back to their u32 original positions (in a CPU with little endianness BGRA bytes will go from right to left : 0xAARRGGBB)
            vec_adjusted.extend_from_slice(&[(alpha << v4_index) + (red_adjusted << v3_index) + (green_adjusted << v2_index) + (blue_adjusted << v1_index)]);
        }
        return vec_adjusted;
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u32> {
        let mut vec_u32_size_set = Vec::<u32>::with_capacity((area_width * area_height * <u32>::units_per_pixel() as usize));
        unsafe { vec_u32_size_set.set_len(vec_u32_size_set.capacity()); }
        return vec_u32_size_set
    }

    fn units_per_pixel() -> u8 { 1 }
    fn get_units_per_pixel(&self) -> u8 { 1 }
}

// Down here are the legacy functions

/// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are retrieved row by row. Returns a Vec of specified type (u8/u32) ( e.g.: let vec_u8 :Vec<u8> = get_bytes(...) ).
pub fn get_bytes<T: PixelValues<T>>(area_width :u32, area_height :u32, src_ul_x :i32, src_ul_y :i32) -> Vec<T> {
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
            src_ul_y
        );

        // create a Vec of u8 values (R,G,B(,A) values range : 0-255), with the size needed to represent the area of the screenshot to take
        //let mut vec :Vec<u8> = vec![0; area_width as usize * area_height as usize * 4];
        
        let mut vec = <T>::initialize_vec(area_width as usize, area_height as usize);
        //get_bitmap_bits(captured_hbmp, &mut vec);
        // add the captured area's pixels RGB values to the Vec
        GetBitmapBits(
            captured_hbmp,
            (area_width * area_height * 4) as i32,
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

/// send Blue Green Red Alpha values to the pixels of a defined area of the screen
/// source_constant_alpha sets the Alpha value of every BGRA (so it sets the whole image's opacity , range : 0-255)
/// set source_constant_alpha to 255 in order to use per-pixel alpha values
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes_legacy<T: PixelValues<T> + Copy>(vec :&Vec<T>, area_width :u32, area_height :u32, dst_ul_x :i32, dst_ul_y :i32, source_constant_alpha :u8) {
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
            vec.as_ptr() as *mut c_void
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

/// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
/// The bytes are sent row by row, the Alpha value in BlueGreenRedAlpha, that is used to define transparency,
/// will be ignored, as it will be max by default (255), so every pixel will have full opacity
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes_alpha_disabled_legacy<T>(vec :&Vec<T>, area_width :u32, area_height :u32, dst_ul_x :i32, dst_ul_y :i32) {
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
            vec.as_ptr() as *mut c_void
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
        bitblock_transfer::bit_block_transfer(
            screen,
            dst_ul_x,
            dst_ul_y,
            area_width, area_height,
            dc_src,
            pixels_upperleftcorner_x,
            pixels_upperleftcorner_y
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
/// bgr_u32_to_hide must not contain A (e.g.: 0x00FF 8080, A must be zero for the hiding to work)
/// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
pub fn send_bytes_alpha_disabled_hide_specific_bgr_legacy<T>(vec :&Vec<T>, area_width :u32, area_height :u32, dst_ul_x :i32, dst_ul_y :i32, hide_b :u8, hide_g :u8, hide_r :u8) {
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
            vec.as_ptr() as *mut c_void
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
            bgr_u32_to_hide.to_owned()
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



// based on
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen