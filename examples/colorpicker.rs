use windows::Win32::{
    Foundation::POINT,
    Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, GetPixel, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
    },
    UI::WindowsAndMessaging::GetCursorPos,
};

fn to_hex(col: u32) -> String {
    let red = ((col >> 16) & 0xff) as u8;
    let green = ((col >> 8) & 0xff) as u8;
    let blue = (col & 0xff) as u8;
    format!("#{:02x}{:02x}{:02x}", red, green, blue)
}

fn get_color_at_cursor(x: i32, y: i32) {
    let hdc = unsafe { GetDC(None) };
    let color = unsafe { GetPixel(hdc, x, y) };
    unsafe { ReleaseDC(None, hdc) };

    let col = to_hex(color.0);
    println!("color:{}", col);
}

fn capture_screen() {
    let mut cursor_pos = POINT::default();
    if unsafe { GetCursorPos(&mut cursor_pos).is_err() } {
        eprintln!("error get cursor");
        return;
    }
    let screen_dc = unsafe { GetDC(None) };
    if screen_dc.is_invalid() {
        eprintln!("failed to get screen dc");
        return;
    }

    let mem_dc = unsafe { CreateCompatibleDC(Some(screen_dc)) };
    if mem_dc.is_invalid() {
        eprintln!("failed to get mem dc");
        return;
    }

    let x = cursor_pos.x;
    let y = cursor_pos.y + 500;
    let width = 1;
    let height = 1; // Capture a 1-pixel-wide, 1-pixel-tall region
    let bitmap = unsafe { CreateCompatibleBitmap(screen_dc, width, height) };
    if bitmap.is_invalid() {
        eprintln!("Failed to create compatible bitmap");
        unsafe { DeleteDC(mem_dc) };
        unsafe { ReleaseDC(None, screen_dc) };
        return;
    }

    // Select the bitmap into the memory DC
    let old_bitmap = unsafe { SelectObject(mem_dc, bitmap.into()) };

    // Perform the bit block transfer
    if unsafe { BitBlt(mem_dc, 0, 0, width, height, Some(screen_dc), x, y, SRCCOPY).is_err() } {
        eprintln!("BitBlt failed");
    }

    // Prepare to extract pixel data
    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height, // Negative to indicate a top-down DIB
            biPlanes: 1,
            biBitCount: 24, // 24 bits per pixel (RGB)
            biCompression: BI_RGB.0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD::default(); 1], // Not used for BI_RGB
    };

    let mut pixel_data = vec![0u8; (width * height * 3) as usize]; // 3 bytes per pixel (RGB)

    // Get pixel data
    if unsafe {
        GetDIBits(
            mem_dc,
            bitmap,
            0,
            height as u32,
            Some(pixel_data.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    } == 0
    {
        eprintln!("GetDIBits failed");
    } else {
        // Convert pixel data to array of colors
        let mut colors = Vec::new();
        for i in (0..pixel_data.len()).step_by(3) {
            let b = pixel_data[i];
            let g = pixel_data[i + 1];
            let r = pixel_data[i + 2];
            colors.push((r, g, b)); // RGB tuple
        }

        // Print the color array
        println!("Captured Colors: {:?}", colors);
    }

    // Cleanup
    unsafe { SelectObject(mem_dc, old_bitmap) };
    unsafe { DeleteObject(bitmap.into()) };
    unsafe { DeleteDC(mem_dc) };
    unsafe { ReleaseDC(None, screen_dc) };
}

fn main() -> windows::core::Result<()> {
    capture_screen();
    Ok(())
    // loop {
    //     let mut cursor_pos = POINT::default();
    //     unsafe {
    //         match GetCursorPos(&mut cursor_pos) {
    //             Ok(_) => {}
    //             Err(_) => panic!("cant find cursor"),
    //         }
    //     }
    //     get_color_at_cursor(cursor_pos.x, cursor_pos.y);

    //     std::thread::sleep(Duration::from_millis(10));
    // }
}
