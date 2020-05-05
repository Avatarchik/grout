use winapi::shared::windef::RECT;
use winapi::um::winuser::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};

use std::mem;

use crate::Window;

/// x & y coordinates are relative to top left of screen
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Rect {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }
    }
}

impl From<Rect> for RECT {
    fn from(rect: Rect) -> Self {
        RECT {
            left: rect.x,
            top: rect.y,
            right: rect.x + rect.width,
            bottom: rect.y + rect.height,
        }
    }
}

pub unsafe fn get_work_area() -> Rect {
    let active_monitor = {
        let active_window = Window::get_foreground();
        MonitorFromWindow(active_window.hwnd, MONITOR_DEFAULTTONEAREST)
    };

    let work_area: Rect = {
        let mut info: MONITORINFO = mem::zeroed();
        info.cbSize = mem::size_of::<MONITORINFO>() as u32;

        GetMonitorInfoW(active_monitor, &mut info);

        info.rcWork.into()
    };

    work_area
}