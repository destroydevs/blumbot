use std::mem;
use winapi::shared::minwindef::{LPARAM, TRUE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetClassNameW, GetWindowRect, GetWindowTextW, IsWindowVisible};

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub hwnd: HWND,
    pub class: String,
    pub title: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub is_visible: bool,
}

impl Default for WindowInfo {
    fn default() -> Self {
        unsafe {
            WindowInfo {
                hwnd: mem::zeroed(),
                class: "".to_string(),
                title: "".to_string(),
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
                is_visible: false,
            }
        }
    }
}

impl WindowInfo {
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
}

pub fn get_all_windows_info() -> Vec<WindowInfo> {
    let mut windows = Vec::new();
    unsafe {
        EnumWindows(Some(enum_proc), &mut windows as *mut _ as LPARAM);
    }
    windows
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let windows = unsafe { &mut *(lparam as *mut Vec<WindowInfo>) };

    if unsafe { IsWindowVisible(hwnd) } != TRUE {
        return 1;
    }

    let mut class_buffer = [0u16; 256];
    let class_len = unsafe {
        GetClassNameW(hwnd, class_buffer.as_mut_ptr(), class_buffer.len() as i32)
    };
    let class = String::from_utf16_lossy(&class_buffer[..class_len as usize]);

    let mut title_buffer = [0u16; 512];
    let title_len = unsafe {
        GetWindowTextW(hwnd, title_buffer.as_mut_ptr(), title_buffer.len() as i32)
    };
    let title = String::from_utf16_lossy(&title_buffer[..title_len as usize]);

    let mut rect = mem::zeroed();
    unsafe { GetWindowRect(hwnd, &mut rect) };

    windows.push(WindowInfo {
        hwnd,
        class,
        title,
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        is_visible: unsafe { IsWindowVisible(hwnd) } == TRUE,
    });

    1
}