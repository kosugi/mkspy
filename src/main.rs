#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};

use windows::core::{w, Result, HSTRING, PCWSTR};
use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::{
        BeginPaint, CreateFontW, DeleteObject, EndPaint, InvalidateRect, SelectObject, TextOutW,
        UpdateWindow, HFONT, PAINTSTRUCT,
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
        RegisterClassW, SetTimer, ShowWindow, CW_USEDEFAULT, HMENU, MSG, SW_SHOW, WM_CREATE,
        WM_DESTROY, WM_PAINT, WM_TIMER, WNDCLASSW, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW,
    },
};

fn choose<T>(condition: bool, lhs: T, rhs: T) -> T {
    if condition {
        lhs
    } else {
        rhs
    }
}

fn is_key_pressed(key: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(key.0.into()) & 0x8000u16 as i16 != 0 }
}

fn is_shift_pressed() -> bool {
    is_key_pressed(VK_SHIFT)
}

fn is_ctrl_pressed() -> bool {
    is_key_pressed(VK_CONTROL)
}

fn is_alt_pressed() -> bool {
    is_key_pressed(VK_MENU)
}

fn is_win_pressed() -> bool {
    is_key_pressed(VK_LWIN) || is_key_pressed(VK_RWIN)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut HFONT: Option<HFONT> = None;

    match message {
        WM_CREATE => {
            HFONT = Some(CreateFontW(
                16,
                0,
                0,
                0,
                400,
                false.into(),
                false.into(),
                false.into(),
                1,
                0,
                0,
                0,
                0,
                w!("Courier New"),
            ));
            LRESULT(0)
        }
        WM_TIMER => {
            InvalidateRect(hwnd, None, true);
            LRESULT(0)
        }
        WM_PAINT => {
            let s = format!(
                "[{}][{}][{}][{}]",
                choose(is_shift_pressed(), "SHIFT", "     "),
                choose(is_ctrl_pressed(), "CTRL", "    "),
                choose(is_alt_pressed(), "ALT", "   "),
                choose(is_win_pressed(), "WIN", "   ")
            );

            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            if let Some(hfont) = HFONT {
                SelectObject(hdc, hfont);
            }
            TextOutW(hdc, 0, 0, HSTRING::from(&s).as_wide());
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            if let Some(hfont) = HFONT.take() {
                DeleteObject(hfont);
            }
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

fn main() -> Result<()> {
    unsafe {
        let Ok(instance) = GetModuleHandleW(PCWSTR::null()) else {
            return Ok(());
        };
        let window_class = w!("ModifierKeySpy");
        let wc = WNDCLASSW {
            hInstance: HINSTANCE(instance.0),
            lpszClassName: window_class,
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST,
            window_class,
            w!("Modifier Key Spy"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            200,
            60,
            HWND(0),
            HMENU(0),
            instance,
            None,
        );
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        SetTimer(hwnd, 1, 100, None);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}
