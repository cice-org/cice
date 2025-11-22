use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use windows::Win32::Graphics::Gdi::{
    GetBitmapBits, GetDeviceCaps, GetWindowDC, HGDIOBJ, HORZRES, VERTRES,
};
use windows::{
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::Gdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, HBITMAP,
            HDC, ReleaseDC, SRCCOPY, SelectObject,
        },
        UI::WindowsAndMessaging::GetWindowRect,
    },
    core::Result,
};

use crate::window::Window;

/// 捕获指定窗口截图并保存为PNG
fn capture_window(window: Window) -> Result<DynamicImage> {
    let hwnd = HWND(window.as_raw_hwnd());
    let mut rect: RECT = RECT::default();
    let _ = unsafe { GetWindowRect(hwnd, &mut rect) }?;
    // let width = (rect.right - rect.left) as u32 * window.dpi() / 96;
    // let height = (rect.bottom - rect.top) as u32 * window.dpi() / 96;

    let window_dc: HDC = unsafe { GetWindowDC(Some(hwnd)) };
    let width = unsafe { GetDeviceCaps(Some(window_dc), HORZRES) };
    let height = unsafe { GetDeviceCaps(Some(window_dc), VERTRES) };
    let mem_dc: HDC = unsafe { CreateCompatibleDC(Some(window_dc)) };
    let hbitmap: HBITMAP =
        unsafe { CreateCompatibleBitmap(window_dc, width as i32, height as i32) };
    unsafe { SelectObject(mem_dc, HGDIOBJ::from(hbitmap)) };

    unsafe {
        let _ = BitBlt(
            mem_dc,
            0,
            0,
            width as i32,
            height as i32,
            Some(window_dc),
            0,
            0,
            SRCCOPY,
        );
    };
    // let bitmap_info = windows::Win32::Graphics::Gdi::BITMAPINFO {
    //     bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
    //         biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>() as u32,
    //         biWidth: width as i32,
    //         biHeight: -(height as i32), // Negative height to get top-down DIB
    //         biPlanes: 1,
    //         biBitCount: 32,
    //         biCompression: BI_RGB.0 as u32,
    //         biSizeImage: 0,
    //         biXPelsPerMeter: 0,
    //         biYPelsPerMeter: 0,
    //         biClrUsed: 0,
    //         biClrImportant: 0,
    //     },
    //     bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD {
    //         rgbBlue: 0,
    //         rgbGreen: 0,
    //         rgbRed: 0,
    //         rgbReserved: 0,
    //     }; 1],
    // };

    let mut rgb_buffer = vec![0u8; (width as u64 * height as u64 * 3) as usize];

    unsafe {
        GetBitmapBits(
            hbitmap,
            rgb_buffer.len() as i32,
            rgb_buffer.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    //Convert to DynamicImage
    let img_buffer: ImageBuffer<Rgb<u8>, _> =
        ImageBuffer::from_raw(width as u32, height as u32, rgb_buffer).unwrap();
    let res = DynamicImage::from(img_buffer);

    // Cleanup Resources
    unsafe {
        let _ = DeleteObject(hbitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, window_dc);
    }

    return Ok(res);
}

#[cfg(test)]
mod tests {

    use super::{super::window::Window, capture_window};

    #[test]
    fn test() {
        let foreground_window = Window::foreground().unwrap();
        if !foreground_window.is_valid() {
            panic!("No valid window");
        }
        let res = capture_window(foreground_window).unwrap();
        debug_assert!(res.width() > 0 && res.height() > 0);
        res.save("./test.png").unwrap();
        println!("Image saved to test.png");
    }
}
