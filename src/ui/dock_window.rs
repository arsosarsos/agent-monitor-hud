use windows::core::{BOOL, PWSTR};
use std::cell::Cell;
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetClientRect, GetWindowTextLengthW, GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible};

thread_local! { static TARGET: Cell<(isize, u32)> = const { Cell::new((0, 0)) }; }

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
            *(data.0 as *mut (isize, u32)) = (hwnd.0 as isize, pid);
            BOOL(0)
        }
    }
    TARGET.with(|target| unsafe {
        let (id, expected_pid) = target.get();
        let mut hwnd = HWND(id as *mut _);
        let mut pid = 0;
        if !hwnd.is_invalid() { GetWindowThreadProcessId(hwnd, Some(&mut pid)); }
        if hwnd.is_invalid() || !IsWindow(Some(hwnd)).as_bool() || pid != expected_pid {
            let mut found = (0isize, 0u32);
            let _ = EnumWindows(Some(visit), LPARAM(&mut found as *mut _ as isize));
            target.set(found);
            hwnd = HWND(found.0 as *mut _);
        }
        if hwnd.is_invalid() || !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() { return None; }
        let mut rect = RECT::default();
        if GetClientRect(hwnd, &mut rect).is_err() || rect.right <= 0 || rect.bottom <= 0 { return None; }
        let mut origin = POINT::default();
        if !ClientToScreen(hwnd, &mut origin).as_bool() { return None; }
        Some((origin.x, origin.y, origin.x + rect.right, origin.y + rect.bottom))
    })
}
