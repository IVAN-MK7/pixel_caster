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

/// Bit-block transfer of the color data corresponding to an area of pixels of the RGBA sequence (from HDC to CreatedHDC).
/// It doesn't print the 4th value (A : alpha,opacity) so only RGB, the A won't be used.
pub fn to_screenshot (dst :&CreatedHDC, dst_ulx_x :&i32, dst_ulx_y :&i32, req_width :&i32, req_height :&i32, src :&HDC, src_ulx_x :&i32, src_ulx_y :&i32) {
    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt
        BitBlt(
            // A handle to the destination device context
            dst.to_owned(),
            // The x-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulx_x.to_owned(),
            // The y-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulx_y.to_owned(),
            // The width, in logical units, of the source and destination rectangles
            req_width.to_owned(),
            // The height, in logical units, of the source and the destination rectangles
            req_height.to_owned(),
            // A handle to the source device context
            src.to_owned(),
            // The x-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulx_x.to_owned(),
            // The y-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulx_y.to_owned(),
            // A raster-operation code. These codes define how the color data for the source rectangle is to be combined with the color data for the destination rectangle to achieve the final color
            SRCCOPY,
        );
    }
}

// maybe check if there is a way to make these 2 bitblock_transfer into one, the issue is in CreatedHDC and HDC as destination(dst) and source(src) and vice versa

/// Bit-block transfer of the color data corresponding to an area of pixels of the RGBA sequence (from CreatedHDC to HDC).
/// It doesn't print the 4th value (A : alpha,opacity) so only RGB, the A won't be used.
pub fn to_screen (dst :&HDC, dst_ulx_x :&i32, dst_ulx_y :&i32, req_width :&i32, req_height :&i32, src :&CreatedHDC, src_ulx_x :&i32, src_ulx_y :&i32) {
    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt
        BitBlt(
            // A handle to the destination device context
            dst.to_owned(),
            // The x-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulx_x.to_owned(),
            // The y-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulx_y.to_owned(),
            // The width, in logical units, of the source and destination rectangles
            req_width.to_owned(),
            // The height, in logical units, of the source and the destination rectangles
            req_height.to_owned(),
            // A handle to the source device context
            src.to_owned(),
            // The x-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulx_x.to_owned(),
            // The y-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulx_y.to_owned(),
            // A raster-operation code. These codes define how the color data for the source rectangle is to be combined with the color data for the destination rectangle to achieve the final color
            SRCCOPY,
        );
    }
}