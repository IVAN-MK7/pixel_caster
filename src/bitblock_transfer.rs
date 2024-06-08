pub use windows::Win32::{
    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/
    Graphics::Gdi::{BitBlt, HDC, SRCCOPY},
};

// CreatedHDC and HDC can be either destination(dst) and source(src) or vice versa
/// Bit-block transfer of the color data corresponding to an area of pixels of the RGBA sequence.
/// It doesn't print the 4th value (A : alpha,opacity) so only RGB, the A won't be used.
pub fn bit_block_transfer(
    dst: HDC,
    dst_ulc_x: i32,
    dst_ulc_y: i32,
    area_width: u32,
    area_height: u32,
    src: HDC,
    src_ulc_x: i32,
    src_ulc_y: i32,
) {
    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt
        match BitBlt(
            // A handle to the destination device context
            dst,
            // The x-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulc_x,
            // The y-coordinate, in logical units, of the upper-left corner of the destination rectangle
            dst_ulc_y,
            // The width, in logical units, of the source and destination rectangles
            area_width as i32,
            // The height, in logical units, of the source and the destination rectangles
            area_height as i32,
            // A handle to the source device context
            src,
            // The x-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulc_x,
            // The y-coordinate, in logical units, of the upper-left corner of the source rectangle
            src_ulc_y,
            // A raster-operation code. These codes define how the color data for the source rectangle is to be combined with the color data for the destination rectangle to achieve the final color
            SRCCOPY,
        ) {
            Ok(_) => (),
            Err(e) => println!("bit_block_transfer error: {}", e),
        };
    }
}
