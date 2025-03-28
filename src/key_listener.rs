use std::sync::mpsc::Sender;
use std::process;

const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: u32 = 0x0100;

pub fn register(c: Sender<bool>) {
    use std::ptr::null_mut;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage, MSG};
    unsafe {
        let h_mod = GetModuleHandleW(null_mut());
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_mod, 0);
        if hook.is_null() {
            return;
        }

        let mut msg: MSG = std::mem::zeroed();

        c.send(true).unwrap();

        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: usize, l_param: isize) -> isize {
    use winapi::um::winuser::KBDLLHOOKSTRUCT;
    if n_code >= 0 && w_param as u32 == WM_KEYDOWN {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;
        if vk_code == 82 {
            process::exit(0);
        }
    }
    0
}