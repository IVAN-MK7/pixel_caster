// https://github.com/microsoft/windows-rs
extern crate windows;

use windows::Win32::{
    UI::WindowsAndMessaging::{
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagea
        LoadImageA,
        IMAGE_BITMAP,
        LR_LOADFROMFILE, 
        LR_CREATEDIBSECTION,
        LR_DEFAULTSIZE,
    },
    Graphics::Gdi::{
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/
        HBITMAP,
        CreateCompatibleDC,
        SelectObject,
        GetDC,
        BitBlt,
        SRCCOPY,
        ReleaseDC,
        DeleteDC,
        DeleteObject
    }
};

// in Cargo.toml : added "alloc" in features to make LoadimageA work
/*[dependencies.windows]
version = "0.35.0"

features = [
    "alloc",
    [..]
]*/

fn main() {
    unsafe {
        match LoadImageA(
            None,
            "P:\\pixel_caster\\media\\Logo_MK7_SignatureS.bmp", // LoadImageA converts path's &str to PSTR
            IMAGE_BITMAP,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::LR_LOADTRANSPARENT | LR_LOADFROMFILE | LR_CREATEDIBSECTION | LR_DEFAULTSIZE,
        ) {
            Ok(res) => { 
                let bmp_isize = res.0; // returns an isize
                // convert the isize into an HBITMAP
                let bmp = HBITMAP(bmp_isize.into());
                let dc_src = CreateCompatibleDC(None);
                let bmp_prev = SelectObject(dc_src, bmp);
            
                let dc_dst = GetDC(None);
        
                BitBlt(
                    dc_dst,
                    0, 
                    0, 
                    300, 
                    200,
                    dc_src,
                    0,
                    0,
                    SRCCOPY,
                );
        
                ReleaseDC(None, dc_dst);
                SelectObject(dc_src, bmp_prev);
                DeleteDC(dc_src);
                DeleteObject(bmp);
            },
            Err(_) => {
                
            },
        }
    }
}