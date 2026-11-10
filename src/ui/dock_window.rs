use windows::core::{BOOL, PWSTR};
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetClientRect, GetWindowTextLengthW, GetWindowThreadProcessId, IsIconic, IsWindowVisible};

/// Find the Codex Desktop window by its packaged executable path, not its title.
pub fn codex_rect() -> Option<(i32, i32, i32, i32)> {
    unsafe extern "system" fn visit(hwnd: HWND, data: LPARAM) -> BOOL {
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() || GetWindowTextLengthW(hwnd) == 0 { return BOOL(1); }
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            let Ok(process) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else { return BOOL(1) };
            let mut path = [0u16; 1024];
            let mut len = path.len() as u32;
            let result = QueryFullProcessImageNameW(process, PROCESS_NAME_FORMAT(0), PWSTR(path.as_mut_ptr()), &mut len);
            let _ = CloseHandle(process);
            if result.is_err() || !String::from_utf16_lossy(&path[..len as usize]).to_ascii_lowercase().contains("\\windowsapps\\openai.codex_") { return BOOL(1); }
            let mut rect = RECT::default();
            if GetClientRect(hwnd, &mut rect).is_err() || rect.right <= 0 || rect.bottom <= 0 { return BOOL(1); }
            let mut origin = POINT::default();
            if !ClientToScreen(hwnd, &mut origin).as_bool() { return BOOL(1); }
            *(data.0 as *mut Option<(i32, i32, i32, i32)>) = Some((origin.x, origin.y, origin.x + rect.right, origin.y + rect.bottom));
            BOOL(0)
        }
    }
    let mut result = None;
    unsafe { let _ = EnumWindows(Some(visit), LPARAM(&mut result as *mut _ as isize)); }
    result
}
