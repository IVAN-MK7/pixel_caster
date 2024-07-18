#![allow(clippy::too_many_arguments)]

// https://github.com/microsoft/windows-rs
#[cfg(target_os = "windows")]
extern crate libc;
#[cfg(target_os = "windows")]
extern crate windows;
#[macro_use]
pub mod macros;

#[cfg(target_os = "linux")]
use std::ffi::{c_uint, c_void};

use image::GenericImageView;
#[cfg(target_os = "windows")]
pub use libc::c_void;
#[cfg(target_os = "windows")]
use windows::Win32::{
    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/
    Graphics::Gdi::{
        AlphaBlend, CreateBitmap, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC,
        DeleteObject, GetBitmapBits, GetDC, ReleaseDC, SelectObject, TransparentBlt, AC_SRC_ALPHA,
        AC_SRC_OVER, BLENDFUNCTION, HBITMAP, HDC,
    },
};

#[cfg(target_os = "windows")]
mod bitblock_transfer;
#[cfg(target_os = "windows")]
pub mod legacy;

#[cfg(target_os = "linux")]
use x11::xlib;
#[cfg(target_os = "linux")]
use x11::xlib::XImage;
#[cfg(target_os = "linux")]
use x11::xlib::{
    XAllPlanes, XDefaultScreen, XDefaultVisual, XDestroyImage, XDisplayHeight, XDisplayWidth,
    XGetImage, ZPixmap,
};
#[cfg(target_os = "linux")]
use x11::xlib::{XCreateImage, XDefaultGC, XPutImage};

pub mod pixels;
pub use crate::pixels::{PixelValues, PixelsCollection, BGRA_INVISIBLE_PIXEL};

pub mod bgra_management;

#[cfg(feature = "pixels_string")]
pub mod pixels_string;

#[cfg(target_os = "linux")]
use x11::xlib::{Display, Window};

#[derive(Clone)]
pub enum PlatformScreen {
    #[cfg(target_os = "windows")]
    Windows {
        /// Gets a handle (H) to a device context (DC) for the client area,
        /// in this case for the entire virtual screen (not just a monitor),
        /// instead of a window (from hwnd value)
        /// this is the handle (H) of a memory device context (DC) to which send data (BGRA colors)
        /// Used either as a handle (H) of a memory device context (DC) to/from which send/capture data (BGRA colors)
        screen: HDC,
        /// Create a compatible bitmap of the requested pixel area (area_width x area_height px).
        /// Used either as a handle (H) of a memory device context (DC) from/to which capture/send data (BGRA colors)
        dc_screen: HDC,
        /// requested pixels' area width and height to be captured
        captured_hbmp: HBITMAP,
        /// Determines if the values are to keep after use or not
        is_static: bool,
    },
    #[cfg(target_os = "linux")]
    Linux {
        display: *mut Display,
        root: Window,
        is_static: bool,
    },
}
unsafe impl Send for PlatformScreen {}
unsafe impl Sync for PlatformScreen {}

/// Contains the values needed to locate the area of the screen to work with
#[derive(Clone)]
pub struct ScreenArea {
    /// X dimension (horizontal) position of the upper left corner of the rectangle that delimits the needed screen area
    upperleftcorner_x: i32,
    /// Y dimension (vertical) position of the upper left corner of the rectangle that delimits the needed screen area
    upperleftcorner_y: i32,
    /// Width of the rectangle that delimits the needed screen area
    width: u32,
    /// Height of the rectangle that delimits the needed screen area
    height: u32,
}

/// Screen is used to get/send color bytes from/to the screen in a straightforward way
#[derive(Clone)]
pub struct Screen<T: PixelValues<T> + Copy> {
    /// PixelsCollection containing color bytes data and info
    pixels: PixelsCollection<T>,
    screen_area: ScreenArea,
    platform_screen: PlatformScreen,
    pixels_send_mode: PixelsSendMode,
}

/// Defines how the BGRA bytes representing pixels' colors sent to the screen must be treated
#[derive(Clone, Copy)]
pub enum PixelsSendMode {
    /// Pixels will be sent with the Alpha channel enabled
    AlphaEnabled,
    /// Pixels will be sent with the Alpha channel Disabled (each BGRA's Alpha value will be sent as 255 (full opacity, no transparency))
    AlphaDisabled,
    /// Pixels will be sent with the Alpha channel Disabled (each BGRA's Alpha value will be sent as 255 (full opacity, no transparency)),
    /// and the color resulting from the given BGR u8 values combination will be sent as fully transparent (no opacity, max transparency)
    AlphaDisabledHideBGR(u8, u8, u8),
    /// Pixels will be sent with the provided u8 Alpha value, instead of their own
    CustomAlpha(u8),
}

impl<T: PixelValues<T> + Copy> Screen<T> {
    /// Initializes a new Screen instance
    pub fn new(
        screen_area_upperleftcorner_x: i32,
        screen_area_upperleftcorner_y: i32,
        area_width: u32,
        area_height: u32,
    ) -> Screen<T> {
        let bytes =
            <T as PixelValues<T>>::initialize_vec(area_width as usize, area_height as usize);
        #[cfg(target_os = "windows")]
        let platform_screen = Self::gen_platform_screen_windows(area_width, area_height, true);

        #[cfg(target_os = "linux")]
        let platform_screen = Self::gen_platform_screen_linux(area_width, area_height);

        Screen {
            pixels: PixelsCollection::<T>::create(area_width as usize, area_height as usize, bytes)
                .unwrap(),
            screen_area: ScreenArea {
                upperleftcorner_x: screen_area_upperleftcorner_x,
                upperleftcorner_y: screen_area_upperleftcorner_y,
                width: area_width,
                height: area_height,
            },
            platform_screen,
            pixels_send_mode: PixelsSendMode::AlphaEnabled,
        }
    }

    pub fn get_platform_screen(&self) -> &PlatformScreen {
        &self.platform_screen
    }

    #[cfg(target_os = "windows")]
    /// Prepares the stuff needed to make the Windows API to manage the screen's pixel's data
    fn gen_platform_screen_windows(
        area_width: u32,
        area_height: u32,
        is_static: bool,
    ) -> PlatformScreen {
        unsafe {
            let screen = GetDC(None);
            PlatformScreen::Windows {
                screen,
                dc_screen: CreateCompatibleDC(screen),
                captured_hbmp: CreateCompatibleBitmap(
                    screen,
                    area_width as i32,
                    area_height as i32,
                ),
                is_static,
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn gen_platform_screen_linux(area_width: u32, area_height: u32) -> PlatformScreen {
        use x11::xlib;

        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);

            PlatformScreen::Linux {
                display,
                root,
                is_static: true,
            }
        }
    }

    /// Returns a reference to its PixelsCollection's bytes Vec
    pub fn get_bytes(&self) -> &Vec<T> {
        &self.pixels.bytes
    }

    /// Returns a reference to its PixelsCollection's bytes Vec
    pub fn get_bytes_mut(&mut self) -> &mut Vec<T> {
        &mut self.pixels.bytes
    }

    /// Returns a reference to its PixelsCollection
    pub fn get_pixels_collection(&self) -> &PixelsCollection<T> {
        &self.pixels
    }

    /// Updates its PixelsCollection's bytes with the BGRA bytes of the Screen's set pixels area
    pub fn scan_area(&mut self) {
        Self::get_bytes_from_screen(
            self.pixels.bytes.as_mut_ptr() as *mut c_void,
            self.screen_area.upperleftcorner_x,
            self.screen_area.upperleftcorner_y,
            self.screen_area.width,
            self.screen_area.height,
            &self.platform_screen,
        )
    }
    /// Updates self.pixels.bytes.
    /// # Safety
    ///
    /// Make sure that the `pixels.bytes` won't be accessed by other threads during the whole duration of this function.
    pub unsafe fn scan_area_interior_mutability(&self) {
        unsafe {
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            Self::get_bytes_from_screen(
                (*mut_ptr).pixels.bytes.as_mut_ptr() as *mut c_void,
                self.screen_area.upperleftcorner_x,
                self.screen_area.upperleftcorner_y,
                self.screen_area.width,
                self.screen_area.height,
                &self.platform_screen,
            )
        }
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
        if vec.len()
            != (self.screen_area.width * self.screen_area.height * <T>::units_per_pixel() as u32)
                as usize
        {
            return Err("Provided Vec has not the correct length".to_string());
        }
        Self::get_bytes_from_screen(
            vec.as_mut_ptr() as *mut c_void,
            self.screen_area.upperleftcorner_x,
            self.screen_area.upperleftcorner_y,
            self.screen_area.width,
            self.screen_area.height,
            &self.platform_screen,
        );
        Ok(())
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
    pub fn scan_area_custom(
        vec: &mut Vec<T>,
        src_ul_x: i32,
        src_ul_y: i32,
        area_width: u32,
        area_height: u32,
    ) -> Result<(), String> {
        //if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        if vec.len() != (area_width * area_height * <T>::units_per_pixel() as u32) as usize {
            return Err("Provided Vec has not the correct length".to_string());
        }
        #[cfg(target_os = "windows")]
        let platform_screen = Self::gen_platform_screen_windows(area_width, area_height, false);

        #[cfg(target_os = "linux")]
        let platform_screen = Self::gen_platform_screen_linux(area_width, area_height);

        Self::get_bytes_from_screen(
            vec.as_mut_ptr() as *mut c_void,
            src_ul_x,
            src_ul_y,
            area_width,
            area_height,
            &platform_screen,
        );
        Ok(())
    }

    /// Sends its PixelsCollection's bytes to the Screen's set pixels area
    pub fn update_area(&mut self) {
        Self::pixels_send_mode_matcher(
            &self.pixels.bytes,
            self.screen_area.upperleftcorner_x,
            self.screen_area.upperleftcorner_y,
            self.screen_area.width,
            self.screen_area.height,
            &self.platform_screen,
            self.pixels_send_mode,
        )
    }

    /// Sends the provided Vec's values to the Screen's set pixels area
    pub fn update_area_from_vec(&mut self, vec: &[T]) {
        Self::pixels_send_mode_matcher(
            vec,
            self.screen_area.upperleftcorner_x,
            self.screen_area.upperleftcorner_y,
            self.screen_area.width,
            self.screen_area.height,
            &self.platform_screen,
            self.pixels_send_mode,
        )
    }

    /// Sends a provided Vec's values to the provided screen area with the given PixelsSendMode, without creating a Screen instance
    pub fn update_area_custom(
        vec: &[T],
        screen_area_upperleftcorner_x: i32,
        screen_area_upperleftcorner_y: i32,
        area_width: u32,
        area_height: u32,
        pixels_send_mode: PixelsSendMode,
    ) {
        #[cfg(target_os = "windows")]
        let platform_screen = Self::gen_platform_screen_windows(area_width, area_height, false);

        #[cfg(target_os = "linux")]
        let platform_screen = Self::gen_platform_screen_linux(area_width, area_height);

        Self::pixels_send_mode_matcher(
            vec,
            screen_area_upperleftcorner_x,
            screen_area_upperleftcorner_y,
            area_width,
            area_height,
            &platform_screen,
            pixels_send_mode,
        )
    }

    fn pixels_send_mode_matcher(
        vec: &[T],
        screen_area_upperleftcorner_x: i32,
        screen_area_upperleftcorner_y: i32,
        area_width: u32,
        area_height: u32,
        platform_screen: &PlatformScreen,
        pixels_send_mode: PixelsSendMode,
    ) {
        match pixels_send_mode {
            PixelsSendMode::AlphaEnabled => Self::send_bytes_to_screen(
                vec,
                screen_area_upperleftcorner_x,
                screen_area_upperleftcorner_y,
                area_width,
                area_height,
                255,
                platform_screen,
            ),
            PixelsSendMode::AlphaDisabled => Self::send_bytes_to_screen_alpha_disabled(
                vec,
                screen_area_upperleftcorner_x,
                screen_area_upperleftcorner_y,
                area_width,
                area_height,
                platform_screen,
            ),
            PixelsSendMode::AlphaDisabledHideBGR(b, g, r) => {
                Self::send_bytes_to_screen_alpha_disabled_hide_specific_bgr(
                    vec,
                    screen_area_upperleftcorner_x,
                    screen_area_upperleftcorner_y,
                    area_width,
                    area_height,
                    platform_screen,
                    b,
                    g,
                    r,
                )
            }
            PixelsSendMode::CustomAlpha(custom_alpha) => {
                if custom_alpha == 255 {
                    Self::send_bytes_to_screen_alpha_disabled(
                        vec,
                        screen_area_upperleftcorner_x,
                        screen_area_upperleftcorner_y,
                        area_width,
                        area_height,
                        platform_screen,
                    );
                } else {
                    Self::send_bytes_to_screen(
                        vec,
                        screen_area_upperleftcorner_x,
                        screen_area_upperleftcorner_y,
                        area_width,
                        area_height,
                        custom_alpha,
                        platform_screen,
                    );
                }
            }
        }
    }

    /// send Blue Green Red Alpha values to the pixels of a defined area of the screen
    /// source_constant_alpha sets the Alpha value of every BGRA (so it sets the whole image's opacity , range : 0-255)
    /// set source_constant_alpha to 255 in order to use per-pixel alpha values
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen(
        vec: &[T],
        dst_ul_x: i32,
        dst_ul_y: i32,
        area_width: u32,
        area_height: u32,
        source_constant_alpha: u8,
        platform_screen: &PlatformScreen,
    ) {
        match platform_screen {
            #[cfg(target_os = "windows")]
            PlatformScreen::Windows {
                screen,
                dc_screen,
                captured_hbmp,
                is_static,
            } => {
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
                    let hbmp_replace = SelectObject(*dc_screen, hbmp_from_bytes);

                    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-blendfunction
                    let bf = BLENDFUNCTION {
                        BlendOp: AC_SRC_OVER as u8,
                        BlendFlags: 1,
                        // Set the SourceConstantAlpha value to 255 (opaque) when you only want to use per-pixel alpha values
                        SourceConstantAlpha: source_constant_alpha,
                        // This flag is set when the bitmap has an Alpha channel (that is, per-pixel alpha).
                        // The Windows API use premultiplied alpha, which means that the Red, Green and Blue channel values must be premultiplied with the Alpha channel value.
                        // For example, if the alpha channel value is x, the Red, Green and Blue channels must be multiplied by x and divided by 0xff (255) prior to the call.
                        AlphaFormat: AC_SRC_ALPHA as u8,
                    };

                    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-alphablend
                    AlphaBlend(
                        //screen,
                        // the entire virtual screen (not just a monitor)
                        *screen,
                        dst_ul_x,
                        dst_ul_y,
                        area_width as i32,
                        area_height as i32,
                        //dc_src,
                        *dc_screen,
                        0,
                        0,
                        area_width as i32,
                        area_height as i32,
                        bf,
                    )
                    .unwrap();

                    // If the PlatformScreen was made for a single run delete it. Must delete the created elements, otherwise after many calls the api will stop working for the whole duration of this .exe process
                    if !is_static {
                        ReleaseDC(None, *screen);
                        // This function returns the previously selected object of the specified type.
                        // An application should always replace a new object with the original,
                        // default object after it has finished drawing with the new object.
                        SelectObject(*dc_screen, hbmp_replace);
                        DeleteDC(*dc_screen).unwrap();
                        DeleteObject(*captured_hbmp).unwrap();
                        DeleteObject(hbmp_from_bytes).unwrap();
                    }
                }
            }
            #[cfg(target_os = "linux")]
            PlatformScreen::Linux { display, root, .. } => {
                use std::ptr;

                unsafe {
                    let screen = x11::xlib::XDefaultScreen(*display);
                    let width = x11::xlib::XDisplayWidth(*display, screen);
                    let height = x11::xlib::XDisplayHeight(*display, screen);

                    // Create an XImage from the pixel buffer
                    let image = x11::xlib::XCreateImage(
                        *display,
                        x11::xlib::XDefaultVisual(*display, screen),
                        24,
                        x11::xlib::ZPixmap,
                        0,
                        vec.as_ptr() as *mut i8,
                        area_width as u32,
                        area_height as u32,
                        32,
                        0,
                    );

                    // Put the image on the screen
                    x11::xlib::XPutImage(
                        *display,
                        *root,
                        x11::xlib::XDefaultGC(*display, screen),
                        image,
                        0,
                        0,
                        dst_ul_x as i32,
                        dst_ul_y as i32,
                        area_width as u32,
                        area_height as u32,
                    );

                    // Cleanup
                    x11::xlib::XDestroyImage(image);
                }
            }
        }
    }

    /// Sends the bytes to the pixels of a screen area of the requested size, starting from an absolute position on the screen.
    /// The bytes are sent row by row, the Alpha value in BlueGreenRedAlpha, that is used to define transparency,
    /// will be ignored, as it will be max by default (255), so every pixel will have full opacity
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen_alpha_disabled(
        vec: &[T],
        dst_ul_x: i32,
        dst_ul_y: i32,
        area_width: u32,
        area_height: u32,
        platform_screen: &PlatformScreen,
    ) {
        match platform_screen {
            #[cfg(target_os = "windows")]
            PlatformScreen::Windows {
                screen,
                dc_screen,
                captured_hbmp,
                is_static,
            } => {
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
                    //let dc_src = CreateCompatibleDC(None);

                    //let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
                    let hbmp_replace = SelectObject(*dc_screen, hbmp_from_bytes);

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
                        *screen,
                        dst_ul_x,
                        dst_ul_y,
                        area_width,
                        area_height,
                        //dc_src,
                        *dc_screen,
                        pixels_upperleftcorner_x,
                        pixels_upperleftcorner_y,
                    );

                    //std::mem::forget(vec);

                    if !is_static {
                        ReleaseDC(None, *screen);
                        // This function returns the previously selected object of the specified type.
                        // An application should always replace a new object with the original,
                        // default object after it has finished drawing with the new object.
                        SelectObject(*dc_screen, hbmp_replace);
                        let _ = DeleteDC(*dc_screen);
                        let _ = DeleteObject(*captured_hbmp);
                        let _ = DeleteObject(hbmp_from_bytes);
                    }
                }
            }
            #[cfg(target_os = "linux")]
            PlatformScreen::Linux { display, root, .. } => {
                use std::ptr;

                unsafe {
                    let screen = XDefaultScreen(*display);
                    let width = XDisplayWidth(*display, screen);
                    let height = XDisplayHeight(*display, screen);

                    // Create an XImage from the pixel buffer
                    let image = XCreateImage(
                        *display,
                        XDefaultVisual(*display, screen),
                        24, // 24 bits per pixel (8 bits each for R, G, B)
                        ZPixmap,
                        0,
                        vec.as_ptr() as *mut i8,
                        area_width,
                        area_height,
                        32, // Bitmap pad
                        0,  // Bytes per line (0 to calculate automatically)
                    );

                    // Put the image on the screen
                    XPutImage(
                        *display,
                        *root,
                        XDefaultGC(*display, screen),
                        image,
                        0,
                        0,
                        dst_ul_x,
                        dst_ul_y,
                        area_width,
                        area_height,
                    );

                    // Cleanup
                    XDestroyImage(image);
                }
            }
        }
    }

    /// send Blue Green Red Alpha (ignored) values to the pixels of a defined area of the screen
    /// and make so that if a BGRA value to be sent to a pixel matches a specific (A is ignored)BGR (u32) value
    /// that color will be sent as completely transparent
    /// e.g., every time a white (B=255, G=255, R=255, A=any_u8_value) is to be sent to a pixel it must be sent as completely transparent, invisible, hidden
    /// bgr_u32_to_hide must not contain A (e.g.: 0x00FF 8080, A must be zero for the hiding to work)
    /// The color chunks must be in BGRA, if Vec<u32> and the CPU has little endian then the color chunks must be in ARGB
    fn send_bytes_to_screen_alpha_disabled_hide_specific_bgr(
        vec: &[T],
        dst_ul_x: i32,
        dst_ul_y: i32,
        area_width: u32,
        area_height: u32,
        platform_screen: &PlatformScreen,
        hide_b: u8,
        hide_g: u8,
        hide_r: u8,
    ) {
        match platform_screen {
            #[cfg(target_os = "windows")]
            PlatformScreen::Windows {
                screen,
                dc_screen,
                captured_hbmp,
                is_static,
            } => {
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
                    //let dc_src = CreateCompatibleDC(None);

                    //let hbmp_replace = SelectObject(dc_src, hbmp_from_bytes);
                    let hbmp_replace = SelectObject(*dc_screen, hbmp_from_bytes);

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
                        *screen,
                        dst_ul_x,
                        dst_ul_y,
                        area_width as i32,
                        area_height as i32,
                        //dc_src,
                        *dc_screen,
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

                    if !is_static {
                        ReleaseDC(None, *screen);
                        // This function returns the previously selected object of the specified type.
                        // An application should always replace a new object with the original,
                        // default object after it has finished drawing with the new object.
                        SelectObject(*dc_screen, hbmp_replace);
                        DeleteDC(*dc_screen).unwrap();
                        DeleteObject(*captured_hbmp).unwrap();
                        DeleteObject(hbmp_from_bytes).unwrap();
                    }
                }
            }
            #[cfg(target_os = "linux")]
            PlatformScreen::Linux { display, root, .. } => {
                use std::ptr;

                // Process the pixels to make specific BGR value fully transparent
                let mut modified_pixels: Vec<u32> = vec
                    .iter()
                    .map(|&pixel| {
                        let pixel = pixel.to_u32();
                        let b = (pixel & 0xFF) as u8;
                        let g = ((pixel >> 8) & 0xFF) as u8;
                        let r = ((pixel >> 16) & 0xFF) as u8;

                        if b == hide_b && g == hide_g && r == hide_r {
                            // Make pixel fully transparent
                            0x00000000
                        } else {
                            // Keep the original pixel value
                            pixel
                        }
                    })
                    .collect();

                unsafe {
                    let screen = XDefaultScreen(*display);
                    let width = XDisplayWidth(*display, screen);
                    let height = XDisplayHeight(*display, screen);

                    // Create an XImage from the modified pixel buffer
                    let image = XCreateImage(
                        *display,
                        XDefaultVisual(*display, screen),
                        24, // 24 bits per pixel (8 bits each for R, G, B)
                        ZPixmap,
                        0,
                        modified_pixels.as_mut_ptr() as *mut i8,
                        area_width,
                        area_height,
                        32, // Bitmap pad
                        0,  // Bytes per line (0 to calculate automatically)
                    );

                    // Put the image on the screen
                    XPutImage(
                        *display,
                        *root,
                        XDefaultGC(*display, screen),
                        image,
                        0,
                        0,
                        dst_ul_x,
                        dst_ul_y,
                        area_width,
                        area_height,
                    );

                    // Cleanup
                    XDestroyImage(image);
                }
            }
        }
    }

    /// Gets the bytes from the pixels of a screen area of the requested size, starting from an absolute position on the screen.
    /// The bytes are retrieved row by row. Return a Vec of specified type ( e.g.: let vec_u8 :Vec<u8> = Vec::get_bytes(...) ).
    fn get_bytes_from_screen(
        self_vec: *mut c_void,
        src_ul_x: i32,
        src_ul_y: i32,
        area_width: u32,
        area_height: u32,
        platform_screen: &PlatformScreen,
    ) {
        match platform_screen {
            #[cfg(target_os = "windows")]
            PlatformScreen::Windows {
                screen,
                dc_screen,
                captured_hbmp,
                is_static,
            } => {
                unsafe {
                    let hbmp_replace = SelectObject(*dc_screen, *captured_hbmp);

                    bitblock_transfer::bit_block_transfer(
                        *dc_screen,
                        0,
                        0,
                        area_width,
                        area_height,
                        *screen,
                        src_ul_x,
                        src_ul_y,
                    );

                    // create a Vec of u32 values (B,G,R,A values range : 0-255), with the size needed to represent the area of the screenshot to take
                    // let mut vec :Vec<u32> = vec![0; area_width.to_owned() as usize * area_height.to_owned() as usize];
                    // let mut vec :Vec<u8> = vec![0; area_width.to_owned() as usize * area_height.to_owned() as usize * 4];

                    // add the captured area's pixels BGRA values to the Vec
                    // DOESENT WORK WITH VEC created with Vec::with_capacity(), only with already populated, e.g.: with vec![;]
                    // to make it work with Vec<T> created with Vec::with_capacity(area_width*area_height*4) must then call vec.set_len(area_width*area_height*4) , which is unsafe
                    GetBitmapBits(
                        //captured_hbmp,
                        *captured_hbmp,
                        (area_width * area_height * <T>::units_per_pixel() as u32) as i32,
                        self_vec, //*vec.as_mut_ptr() as *mut c_void
                    );

                    // If the were made for a single run delete them. Must delete the created elements, otherwise after many calls the api will stop working for the whole duration of this .exe process
                    if !is_static {
                        ReleaseDC(None, *dc_screen);
                        ReleaseDC(None, *screen);
                        // This function returns the previously selected object of the specified type.
                        // An application should always replace a new object with the original,
                        // default object after it has finished drawing with the new object.
                        SelectObject(*dc_screen, hbmp_replace);
                        DeleteDC(*dc_screen).unwrap();
                        DeleteObject(*captured_hbmp).unwrap();
                    }
                }
            }
            #[cfg(target_os = "linux")]
            PlatformScreen::Linux { display, root, .. } => {
                use std::slice;

                unsafe {
                    // Get the image from the screen
                    let ximage = XGetImage(
                        *display,
                        *root,
                        src_ul_x,
                        src_ul_y,
                        area_width,
                        area_height,
                        XAllPlanes(),
                        ZPixmap,
                    );

                    if ximage.is_null() {
                        eprintln!("Unable to get the image from the screen");
                        xlib::XCloseDisplay(*display);
                        return;
                    }

                    // Calculate the number of bytes per pixel (assuming 24-bit color depth)
                    let bytes_per_pixel = 4; // Since we're handling 32-bit (BGRA), even though only 24 bits are used


                    // Get the raw image data
                    let raw_data_ptr = (*ximage).data as *mut u8;
                    let raw_data_len = (area_width * area_height * bytes_per_pixel as u32) as usize;

                    // Create a slice from the raw data pointer
                    let raw_data_slice = slice::from_raw_parts_mut(raw_data_ptr, raw_data_len);

                    /*// This code results in a bytes collection with the Alpha channel of value 0 for each pixel,
                    // since the data retrieve from the screen does not have Alpha channel information.
                    // Copy the data into the provided buffer
                    let self_vec_slice =
                        slice::from_raw_parts_mut(self_vec as *mut u8, raw_data_len);
                    self_vec_slice.copy_from_slice(raw_data_slice);*/

                    // Create a slice from self_vec
                    let self_vec_slice = slice::from_raw_parts_mut(self_vec as *mut u8, raw_data_len);

                    // Copy the data into the provided buffer and set alpha channel to 255
                    for (i, chunk) in raw_data_slice.chunks_exact(bytes_per_pixel).enumerate() {
                        let base_index = i * bytes_per_pixel;
                        self_vec_slice[base_index] = chunk[0]; // Blue
                        self_vec_slice[base_index + 1] = chunk[1]; // Green
                        self_vec_slice[base_index + 2] = chunk[2]; // Red
                        self_vec_slice[base_index + 3] = 255; // Alpha set to 255 (opaque)
                    }

                    // Cleanup
                    XDestroyImage(ximage);
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    /// Copies the pixels from a given area of the screen and pastes them onto another given area of the screen.
    pub fn copy_and_paste_pixels(
        src_ulc_x: i32,
        src_ulc_y: i32,
        area_width: u32,
        area_height: u32,
        dst_ulc_x: i32,
        dst_ulc_y: i32,
    ) {
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
            let captured_hbmp =
                CreateCompatibleBitmap(screen, area_width as i32, area_height as i32);

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
                src_ulc_y,
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
                pixels_upperleftcorner_y,
            );

            ReleaseDC(None, captured_screen);
            ReleaseDC(None, screen);
            // This function returns the previously selected object of the specified type.
            // An application should always replace a new object with the original,
            // default object after it has finished drawing with the new object.
            SelectObject(captured_screen, hbmp_replace);
            DeleteDC(captured_screen).unwrap();
            DeleteObject(captured_hbmp).unwrap();
        }
    }

    #[cfg(target_os = "linux")]
    /// Copies the pixels from a given area of the screen and pastes them onto another given area of the screen.
    pub fn copy_and_paste_pixels(
        src_ulc_x: i32,
        src_ulc_y: i32,
        area_width: u32,
        area_height: u32,
        dst_ulc_x: i32,
        dst_ulc_y: i32,
    ) {
        use std::{ptr, slice};

        unsafe {
            // Open display
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                eprintln!("Failed to open X display.");
                return;
            }
    
            // Get screen and root window
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
    
            // Get image from source area
            let src_image = xlib::XGetImage(
                display,
                root,
                src_ulc_x,
                src_ulc_y,
                area_width,
                area_height,
                xlib::XAllPlanes(),
                xlib::ZPixmap,
            );
    
            if src_image.is_null() {
                eprintln!("Failed to capture source screen area.");
                xlib::XCloseDisplay(display);
                return;
            }
    
            // Set alpha channel to fully opaque
            let bytes_per_pixel = ((*src_image).bits_per_pixel as u32) / 8;
            let data_length = (area_width * area_height * bytes_per_pixel) as usize;
            let data_slice = slice::from_raw_parts_mut((*src_image).data as *mut u8, data_length);

            for pixel in data_slice.chunks_mut(bytes_per_pixel as usize) {
                if bytes_per_pixel == 4 {
                    // Assuming the format is ARGB or RGBA
                    pixel[3] = 255; // Set alpha channel to 255 (fully opaque)
                }
            }

            // Force synchronization with the X server
            xlib::XSync(display, 0);
            
            // Put image into destination area
            let gc = xlib::XDefaultGC(display, screen);
            xlib::XPutImage(
                display,
                root,
                gc,
                src_image,
                0,
                0,
                dst_ulc_x,
                dst_ulc_y,
                area_width,
                area_height,
            );
    
            // Cleanup
            xlib::XDestroyImage(src_image);
            xlib::XCloseDisplay(display);
        }
    }
}

// based on
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen


// TODO: delete. Random stuff for cfg target_os.
#[cfg(target_os = "windows")]
mod platform {
    use crate::Screen;

    pub fn capture_screen() -> Option<Vec<u8>> {
        let mut screen_u8: Screen<u8> = Screen::new(0, 0, 2560, 1440);
        screen_u8.scan_area();
        return Some(screen_u8.get_bytes().clone());
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use std::ffi::CString;
    use std::ptr;
    use x11::xlib;
    use std::slice;
    use image::{ImageBuffer, Rgb};

    pub fn capture_screen() -> Option<Vec<u8>> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return None;
            }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let width = xlib::XDisplayWidth(display, screen);
            let height = xlib::XDisplayHeight(display, screen);

            let mut attributes: xlib::XWindowAttributes = std::mem::zeroed();
            xlib::XGetWindowAttributes(display, root, &mut attributes);

            let ximage = xlib::XGetImage(
                display,
                root,
                0,
                0,
                width as u32,
                height as u32,
                xlib::XAllPlanes(),
                xlib::ZPixmap,
            );
            if ximage.is_null() {
                xlib::XCloseDisplay(display);
                return None;
            }

            // Extract pixel data
            let pixels = unsafe {
                slice::from_raw_parts(
                    (*ximage).data as *const u8,
                    ((*ximage).width * (*ximage).height * 4) as usize,
                )
            };

            // Create an image buffer
            let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);

            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let idx = (y as usize * (unsafe { *ximage }).width as usize + x as usize) * 4;
                let b = pixels[idx];
                let g = pixels[idx + 1];
                let r = pixels[idx + 2];
                *pixel = Rgb([r, g, b]);
            }

            // Save the image as a PNG file
            img.save("screenshot_org.png").expect("Failed to save the image");

            xlib::XDestroyImage(ximage);
            xlib::XCloseDisplay(display);

            Some(pixels.to_vec())
        }
    }
}
#[test]
fn testinolinux() {
    if let Some(vec) = platform::capture_screen() {
        image::save_buffer_with_format(
            "test_retrieve_test.png",
            &vec,
            2560,
            1440,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_get_pixels() {
    use x11::xlib;
    use std::ptr;
    use std::slice;
    use image::{ImageBuffer, Rgb};
    // Open the connection to the X server
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("Unable to open X display");
        return;
    }

    // Get the root window (the entire screen)
    let screen = unsafe { xlib::XDefaultScreen(display) };
    let root_window = unsafe { xlib::XRootWindow(display, screen) } as Window;

    // Get the screen dimensions
    let width = unsafe { xlib::XDisplayWidth(display, screen) };
    let height = unsafe { xlib::XDisplayHeight(display, screen) };

    // Capture the screen
    let ximage = unsafe {
        xlib::XGetImage(
            display,
            root_window,
            0,
            0,
            width as u32,
            height as u32,
            !0,
            xlib::ZPixmap,
        )
    };

    if ximage.is_null() {
        eprintln!("Unable to get the image from the screen");
        unsafe {
            xlib::XCloseDisplay(display);
        }
        return;
    }

    // Extract pixel data
    let pixels = unsafe {
        slice::from_raw_parts(
            (*ximage).data as *const u8,
            ((*ximage).width * (*ximage).height * 4) as usize,
        )
    };

    // Create an image buffer
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = (y as usize * (unsafe { *ximage }).width as usize + x as usize) * 4;
        let b = pixels[idx];
        let g = pixels[idx + 1];
        let r = pixels[idx + 2];
        *pixel = Rgb([r, g, b]);
    }

    // Save the image as a PNG file
    img.save("screenshot.png").expect("Failed to save the image");

    // Clean up
    unsafe {
        xlib::XDestroyImage(ximage);
        xlib::XCloseDisplay(display);
    }

    println!("Screenshot saved as screenshot.png");
}

// TODO: fix, don't know why but the XPutImage doesnt work (on Ubuntu 22.04)
#[cfg(target_os = "linux")]
#[test]
fn test_put_pixels() {
    use x11::xlib::*;
    use std::ptr;
    unsafe {
        /*// Open display
        /*let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Failed to open X display");
        }*/
        let display_name = std::ffi::CString::new(":1").unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr()) };
        if display.is_null() {
            panic!("Unable to open X display");
        }
    
        // Get default screen and root window
        let screen_num = xlib::XDefaultScreen(display);
        println!("screen_num: {}", screen_num);
        //let root_window = xlib::XRootWindow(display, screen_num);
        let root_window = unsafe { XDefaultRootWindow(display) };
        println!("root_window: {}", root_window);
    
        // Define image properties (example data)
        let width = 100;
        let height = 100;
        let depth = xlib::XDefaultDepth(display, screen_num) as usize;
        let mut data: Vec<u8> = vec![200; width * height * (depth / 8)];
    
        // Modify 'data' with your actual image bytes
    
        let visual = xlib::XDefaultVisual(display, screen_num);
        
        // Create XImage
        let image = xlib::XCreateImage(
            display,
            visual,
            depth as u32,
            xlib::ZPixmap,
            0,
            data.as_mut_ptr() as *mut std::os::raw::c_char,
            width as u32,
            height as u32,
            32,
            0,
        );
        
        if image.is_null() {
            panic!("Failed to create XImage");
        }
    
        let gc = xlib::XDefaultGC(display, screen_num);
        // Put image on the screen
        xlib::XPutImage(
            display,
            root_window,
            gc,
            image,
            0,
            0,
            0,
            0,
            width as u32,
            height as u32,
        );
    
        // Flush display
        xlib::XFlush(display);
    
        // Close display
        xlib::XCloseDisplay(display);*/

        
        /*// Initialize X11 display
        let display_name = std::ffi::CString::new(":1").unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr()) };
        if display.is_null() {
            panic!("Unable to open X display");
        }

        let screen = unsafe { xlib::XDefaultScreen(display) };
        let root_window = unsafe { xlib::XRootWindow(display, screen) };

        // Load the image
        let img = image::open("image.png").expect("Failed to open image");
        let (img_width, img_height) = img.dimensions();
        let img_rgba = img.to_rgba8();
        let img_data = img_rgba.into_raw();

        // Create XImage
        let ximage = unsafe {
            xlib::XCreateImage(
                display,
                ptr::null_mut(),
                24,
                xlib::ZPixmap,
                0,
                img_data.as_ptr() as *mut i8,
                img_width as c_uint,
                img_height as c_uint,
                32,
                0
            )
        };

        if ximage.is_null() {
            panic!("XCreateImage failed");
        }

        // Display the image on the root window
        let gc = unsafe { xlib::XCreateGC(display, root_window, 0, ptr::null_mut()) };
        unsafe {
            xlib::XPutImage(
                display,
                root_window,
                gc,
                ximage,
                0, 0, 0, 0,
                img_width as c_uint,
                img_height as c_uint
            );
            xlib::XFlush(display);
        }


        // Cleanup
        unsafe {
            xlib::XFreeGC(display, gc);
            xlib::XCloseDisplay(display);
        }*/
        
        
        // Open a connection to the X server
        let display_name = std::ffi::CString::new(":1").unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr()) };
        if display.is_null() {
            panic!("Unable to open X display");
        }

        let screen_num = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen_num);

        // Create a white image
        let depth = XDefaultDepth(display, screen_num);
        let white_pixel = XWhitePixel(display, screen_num);
        let black_pixel = XBlackPixel(display, screen_num);

        let mut image = XCreateImage(
            display,
            XDefaultVisual(display, screen_num),
            depth as u32,
            ZPixmap,
            0,
            std::ptr::null_mut(),
            100,
            100,
            32,
            0,
        );

        (*image).data = Box::into_raw(Box::new(vec![white_pixel as u8; 100 * 100 * 4])) as *mut _;

        // Put the image onto the screen
        XPutImage(
            display,
            root_window,
            XDefaultGC(display, screen_num),
            image,
            0,
            0,
            0,
            0,
            100,
            100,
        );

        // Flush the commands to the X server
        XFlush(display);

        // Clean up resources
        XDestroyImage(image);
        XCloseDisplay(display);
    }
}