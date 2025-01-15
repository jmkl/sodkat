use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

pub fn get_foreground_app() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == std::ptr::null_mut() {
            println!("No foreground window found");
            return None;
        }
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        if process_id == 0 {
            return None;
        }

        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        );

        if let Ok(handle) = process_handle {
            let mut buffer = vec![0u16; MAX_PATH as usize];
            let mut size = buffer.len() as u32;

            if QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_WIN32,
                PWSTR(buffer.as_mut_ptr()),
                &mut size,
            )
            .is_ok()
            {
                let process_name = String::from_utf16_lossy(&buffer[..size as usize]);

                if let Some(exe_name) = process_name.split('\\').last() {
                    Some(exe_name.to_string().to_lowercase())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
