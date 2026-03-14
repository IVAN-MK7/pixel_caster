use std::ptr;
use winapi::ctypes::c_void;
use winapi::shared::windef::{HBITMAP, HDC};
use winapi::um::wingdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CreateCompatibleDC, CreateDIBSection,
    DIB_RGB_COLORS, DeleteDC, DeleteObject, SRCCOPY, SelectObject,
};
use winapi::um::winuser::{GetDC, GetDesktopWindow, ReleaseDC};

pub struct ZeroCopyScreen {
    screen_dc: HDC,
    mem_dc: HDC,
    hbmp: HBITMAP,
    old_hbmp: winapi::shared::windef::HGDIOBJ,
    pub width: i32,
    pub height: i32,
    pub pixels_ptr: *mut u8,
}

impl ZeroCopyScreen {
    /// # Safety
    ///
    /// Calls to unsafe Windows API.
    pub unsafe fn new(width: i32, height: i32) -> Self {
        unsafe {
            let hwnd = GetDesktopWindow();
            let screen_dc = GetDC(hwnd);
            let mem_dc = CreateCompatibleDC(screen_dc);

            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    // A negative height tells Windows to make this a "Top-Down" bitmap.
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32, // BGRA format
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [winapi::um::wingdi::RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }; 1],
            };

            let mut pixels_ptr: *mut c_void = ptr::null_mut();
            let hbmp = CreateDIBSection(
                screen_dc,
                &bmi as *const BITMAPINFO,
                DIB_RGB_COLORS,
                &mut pixels_ptr, // Windows writes the shared memory address here
                ptr::null_mut(),
                0,
            );

            let old_hbmp = SelectObject(mem_dc, hbmp as *mut _);

            Self {
                screen_dc,
                mem_dc,
                hbmp,
                old_hbmp,
                width,
                height,
                pixels_ptr: pixels_ptr as *mut u8, // Cast to a usable byte pointer
            }
        }
    }

    /// Takes the screenshot, directly into shared memory without copying data.
    ///
    /// # Safety
    ///
    /// Do not access its fields while this is being performed.
    pub unsafe fn capture(&self, src_x: i32, src_y: i32) {
        unsafe {
            BitBlt(
                self.mem_dc,
                0,
                0,
                self.width,
                self.height,
                self.screen_dc,
                src_x,
                src_y,
                SRCCOPY,
            );
            // `self.pixels_ptr` now contains the updated BGRA data.
        }
    }

    /// # Safety
    ///
    /// Do not call `capture` while the returned slice is still in use.
    pub unsafe fn as_slice(&self) -> &[u8] {
        let len = (self.width * self.height * 4) as usize;
        unsafe { std::slice::from_raw_parts(self.pixels_ptr, len) }
    }
}

/// Cleans up OS resources when dropped.
impl Drop for ZeroCopyScreen {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.mem_dc, self.old_hbmp);
            DeleteObject(self.hbmp as *mut _);
            DeleteDC(self.mem_dc);
            ReleaseDC(GetDesktopWindow(), self.screen_dc);
        }
    }
}

unsafe impl Send for ZeroCopyScreen {}

unsafe impl Sync for ZeroCopyScreen {}
