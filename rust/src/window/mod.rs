#![allow(dead_code)]

// prevent console from opening
#![windows_subsystem = "windows"]

// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/libloaderapi/index.html?search=winuser
#[cfg(windows)] extern crate winapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::{null_mut};

use self::winapi::um::libloaderapi::GetModuleHandleW;
use self::winapi::um::winuser::{
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    IsWindow,
    GetDC,
    GetClientRect,
    GetWindowRect,
    TranslateMessage,
    DispatchMessageW,
    PeekMessageW,
    BeginDeferWindowPos,
    DeferWindowPos,
    EndDeferWindowPos,
    TrackMouseEvent,
    GetCursorPos,
    GetAsyncKeyState
};
use self::winapi::um::winuser::{
    MSG,
    WNDCLASSW,
    CS_OWNDC,
    CS_HREDRAW,
    CS_VREDRAW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
    PM_REMOVE,
    HOVER_DEFAULT,
    VK_LBUTTON,
};
// windows messages
use self::winapi::um::winuser::{
    WM_MOUSEMOVE,
    WM_MOUSELEAVE,
    WM_RBUTTONDOWN,
    WM_RBUTTONUP,
    WM_LBUTTONDOWN,
    WM_LBUTTONUP,
    WM_NCLBUTTONDOWN,
    WM_NCLBUTTONUP,
    WM_NCMOUSEMOVE,
    WM_NCMOUSELEAVE,
    WM_SYSCOMMAND,
    SC_SIZE,
    SWP_DRAWFRAME,
    SWP_NOOWNERZORDER,
    TME_LEAVE,
    TME_NONCLIENT,
};
// windows nc hit values
use self::winapi::um::winuser::{
    HTLEFT,
    HTRIGHT,
    HTTOP,
    HTTOPLEFT,
    HTTOPRIGHT,
    HTBOTTOM,
    HTBOTTOMLEFT,
    HTBOTTOMRIGHT,
    TRACKMOUSEEVENT,
};
use self::winapi::um::wingdi::{
    StretchDIBits,
    SRCCOPY,
    BITMAPINFO,
    BITMAPINFOHEADER,
    DIB_RGB_COLORS,
    BI_RGB,
};
use self::winapi::um::winnt::{
    VOID,
    PAGE_READWRITE,
    MEM_RESERVE,
    MEM_COMMIT,
    MEM_RELEASE,
};
use self::winapi::um::memoryapi::{
    VirtualAlloc,
    VirtualFree,
};

use winapi::shared::minwindef::{
    DWORD,
    LPARAM,
    LRESULT,
    UINT,
};
use self::winapi::shared::basetsd::{
    SIZE_T,
};
use self::winapi::shared::windef::{
    HWND,
    HDC,
    RECT,
    POINT,
};

use std::os::raw::c_int;
use std::io::Error;

// export color module
pub mod color;
use color::Color;

static mut WINDOWCOUNT: u32 = 0;

#[cfg(windows)]
pub struct WindowBuilder{
    title: Vec<u16>,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int
}

//#region WindowBuilder
#[cfg(windows)]
impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            title: win_32_string("New Window")
        }
    }

    pub fn set_x(&mut self, x: i32) -> &mut Self {
        self.x = x as c_int;
        self
    }

    pub fn set_y(&mut self, y: i32) -> &mut Self {
        self.y = y as c_int;
        self
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> &mut Self {
        self.set_x(x);
        self.set_y(y);
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Self {
        self.width = width as c_int;
        self
    }

    pub fn set_height(&mut self, height: i32) -> &mut Self {
        self.height = height as c_int;
        self
    }

    pub fn set_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.set_width(width);
        self.set_height(height);
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = win_32_string(title);
        self
    }

    pub fn build(&self) -> Window {
        unsafe {
            // hInstance gets a handle to the instance of the window class
            let hinstance = GetModuleHandleW(null_mut());

            let class_name = win_32_string(
                &format!("window_{}", WINDOWCOUNT)
            );

            WINDOWCOUNT += 1;
    
            // create the window class
            let wnd_class = WNDCLASSW {
                style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: hinstance, // instance handle for the window
                lpszClassName: class_name.as_ptr(),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
            };
    
            let register_result = RegisterClassW(&wnd_class);

            if register_result == 0 {
                panic!("{}", Error::last_os_error());
            }
    
            // create a display window from the registered window class
            // https://msdn.microsoft.com/en-us/library/windows/desktop/ms632680(v=vs.85).aspx
            let handle = CreateWindowExW(
                0,
                class_name.as_ptr(),
                self.title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                self.x, // x
                self.y, // y
                self.width, // width
                self.height, // height
                null_mut(), //hWindParent
                null_mut(), // hMenu
                hinstance,
                null_mut() // lpParam
            );
    
            if handle.is_null() {
                panic!("{}", Error::last_os_error());
            }

            let (client_width, client_height) = Window::get_client_size_from_handle(handle);
            let video_memory_size = (client_width * client_height * 4) as SIZE_T;
            let video_memory_pointer = VirtualAlloc(
                null_mut(),
                video_memory_size, // * 4 due to there being 4 bytes per pixel
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE
            );

            if video_memory_pointer.is_null() {
                panic!("{}", Error::last_os_error());
            }
            
            Window {
                handle,
                device_context: GetDC(handle),
                video_memory_pointer,
                bitmap_info: generate_bitmap_info(client_width, client_height),
                minimum_size: (10, Window::get_taskbar_height_from_handle(handle)),
                background_color: Color::WHITE,
                update_state: UpdateState::new(handle),
            }
        }
    }
}
//#endregion

struct UpdateState {
    nc_tracker: TRACKMOUSEEVENT,
    w_tracker: TRACKMOUSEEVENT,
    sizing_direction: LRESULT,
    cached_cursor_pos: (i32, i32),
    cancel_draw: bool,
}

impl UpdateState {
    fn new(handle: HWND) -> Self {
        Self {
            nc_tracker: TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as DWORD,
                dwFlags: TME_LEAVE | TME_NONCLIENT,
                hwndTrack: handle,
                dwHoverTime: HOVER_DEFAULT
            },
            w_tracker: TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as DWORD,
                dwFlags: TME_LEAVE,
                hwndTrack: handle,
                dwHoverTime: HOVER_DEFAULT
            },
            sizing_direction: 0,
            cached_cursor_pos: (0, 0),
            cancel_draw: false
        }
    }

    fn get_sizing_direction(&self) -> LRESULT {
        self.sizing_direction
    }

    fn set_sizing_direction(&mut self, direction: LRESULT) {
        self.sizing_direction = direction;
    }

    fn clear_sizing_direction(&mut self) {
        self.set_sizing_direction(0);
    }

    fn get_cached_cursor_pos(&self) -> (i32, i32) {
        self.cached_cursor_pos
    }

    fn cache_cursor_pos(&mut self, pos: (i32, i32)) {
        self.cached_cursor_pos = pos;
    }

    fn drawing_enabled(&self) -> bool {
        !self.cancel_draw
    }

    fn enable_draw(&mut self) {
        self.cancel_draw = false;
    }

    fn cancel_draw(&mut self) {
        self.cancel_draw = true;
    }

    fn track_mouse(&mut self) {
        unsafe {
            let nc_result = TrackMouseEvent(&mut self.nc_tracker);
            let w_result = TrackMouseEvent(&mut self.w_tracker);
            if nc_result == 0 || w_result == 0 {
                panic!("{}", Error::last_os_error());
            }
        }
    }
}

#[cfg(windows)]
pub struct Window {
    handle: HWND,
    device_context: HDC,
    video_memory_pointer: *mut VOID,
    bitmap_info: BITMAPINFO,
    minimum_size: (i32, i32),
    background_color: Color,
    update_state: UpdateState
}

#[cfg(windows)]
impl Window {
    pub fn is_running(&self) -> bool {
        unsafe { IsWindow(self.handle) != 0 }
    }

    fn is_resizing(&mut self) -> bool {
        if unsafe{ GetAsyncKeyState(VK_LBUTTON) } as u16 & 0x8000 == 0x8000 && self.update_state.get_sizing_direction() != 0 {
            true
        }
        else {
            self.update_state.clear_sizing_direction();
            false
        }
    }

    fn defer_window(&mut self, x: i32, y: i32, width: i32, height: i32, flags: UINT) {
        unsafe {
            let begin_defer = BeginDeferWindowPos(1);
            let defer = DeferWindowPos(
                begin_defer,
                self.handle,
                null_mut(),
                x,
                y,
                width,
                height,
                flags
            );
            let result = EndDeferWindowPos(defer);
            if result == 0 {
                panic!("{}", Error::last_os_error());
            }
        }
    }

    fn update_bitmap(&mut self) {
        let (client_width, client_height) = self.get_client_size();
        // println!("New bmap size: ({}, {})", client_width, client_height);
        unsafe {
            // ensure the memory from the last section of video memory is freed
            let free_result = VirtualFree(
                self.video_memory_pointer,
                0,
                MEM_RELEASE
            );
            if free_result == 0 {
                panic!("{}", Error::last_os_error());
            }
            let video_memory_pointer = VirtualAlloc(
                null_mut(),
                (client_width * client_height * 4) as SIZE_T, // * 4 due to there being 4 bytes per pixel
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE
            );
            if self.video_memory_pointer.is_null() {
                panic!("{}", Error::last_os_error());
            }
            self.bitmap_info = generate_bitmap_info(client_width, client_height);
            self.video_memory_pointer = video_memory_pointer;
            self.fill(self.background_color);
        }
    }

    fn handle_resize(&mut self) {
        let (cursor_x, cursor_y) = get_cursor_pos();
        // ensure the cursor has moved
        if self.update_state.get_cached_cursor_pos() != (cursor_x, cursor_y) {
            let window_rect = self.get_window_rect();
            let (mut dx, mut dy) = (0, 0);
            let (mut dwidth, mut dheight) = match self.update_state.get_sizing_direction() {
                HTTOP => (0, window_rect.top - cursor_y), // needs translate
                HTBOTTOM => (0, cursor_y - window_rect.bottom),
                HTLEFT => (window_rect.left - cursor_x, 0), // needs translate
                HTRIGHT => (cursor_x - window_rect.right, 0),
                HTTOPLEFT => (window_rect.left - cursor_x, window_rect.top - cursor_y), // needs double translate
                HTTOPRIGHT => (cursor_x - window_rect.right, window_rect.top - cursor_y), // needs translate
                HTBOTTOMLEFT => (window_rect.left - cursor_x, cursor_y - window_rect.bottom), // needs translate
                HTBOTTOMRIGHT => (cursor_x - window_rect.right, cursor_y - window_rect.bottom),
                _ => (0, 0)
            };
            // second round of matching to assign dx and dy
            match self.update_state.get_sizing_direction() {
                HTTOP => dy = dheight,
                HTLEFT => dx = dwidth,
                HTTOPLEFT => {
                    dx = dwidth;
                    dy = dheight;
                },
                HTTOPRIGHT => dy = dheight,
                HTBOTTOMLEFT => dx = dwidth,
                _ => {}
            }
            let (width, height) = self.get_window_size();
            // dx and dy are used to allow resizing using the top and left borders (remove to see the behaviour this prevents)
            if self.minimum_size.0 >= width + dwidth {
                dwidth = 0;
            }
            if self.minimum_size.1 >= height + dheight {
                dheight = 0;
            }
            self.defer_window(
                window_rect.left - dx,
                window_rect.top - dy,
                width + dwidth,
                height + dheight,
                SWP_DRAWFRAME | SWP_NOOWNERZORDER
            );
            self.update_state.cache_cursor_pos((cursor_x, cursor_y));
            self.update_bitmap();
            // self.draw_screen();
            self.update_state.cancel_draw();
        }
    }

    pub fn handle_messages(&mut self) {
        unsafe {
            // only track the cursor if the window is being resized
            if self.is_resizing() {
                self.update_state.track_mouse();
            }
            let message = mem::MaybeUninit::<MSG>::uninit();
            if PeekMessageW(message.as_ptr() as *mut MSG, self.handle, 0, 0, PM_REMOVE) != 0 {
                let message_code = (*(message.as_ptr())).message;
                let _l_param = (*(message.as_ptr())).lParam;
                let w_param = (*(message.as_ptr())).wParam;
                match message_code {
                    // client area events
                    WM_MOUSEMOVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                        }
                    },
                    WM_MOUSELEAVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                        }
                    },

                    WM_RBUTTONDOWN | WM_RBUTTONUP => {}, // handle events
                    WM_LBUTTONDOWN | WM_LBUTTONUP => {}, // handle events

                    // nc events (taskbar, resizing, syscommand etc)
                    WM_NCLBUTTONDOWN => {
                        match w_param as isize {
                            HTTOPLEFT | HTTOPRIGHT |
                            HTBOTTOMLEFT | HTBOTTOMRIGHT |
                            HTTOP | HTRIGHT |
                            HTBOTTOM | HTLEFT => {
                                self.update_state.set_sizing_direction(w_param as LPARAM);
                                self.update_state.cache_cursor_pos(get_cursor_pos());
                            },
                            _ => {
                                TranslateMessage(message.as_ptr() as *const MSG);
                                DispatchMessageW(message.as_ptr() as *const MSG);
                            }
                        }
                    },
                    WM_NCLBUTTONUP => {
                        match w_param as isize {
                            _ => {
                                TranslateMessage(message.as_ptr() as *const MSG);
                                DispatchMessageW(message.as_ptr() as *const MSG);
                            }
                        }
                    },
                    WM_NCMOUSEMOVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                        }
                    },
                    WM_NCMOUSELEAVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                        }
                    },
                    WM_SYSCOMMAND => {
                        match w_param {
                            SC_SIZE => { println!("SC Sizing"); },
                            _ => {
                                TranslateMessage(message.as_ptr() as *const MSG);
                                DispatchMessageW(message.as_ptr() as *const MSG);
                            }
                        }
                    },
                    _ => {
                        println!("Uncaught: {}", (*(message.as_ptr())).message);
                        TranslateMessage(message.as_ptr() as *const MSG);
                        DispatchMessageW(message.as_ptr() as *const MSG);
                    }
                }
            }
        }
    }

    // draws the window and handles any messages
    pub fn update(&mut self) {
        // ensure the screen is drawn at least every other frame without interference
        // of windows messages (used to avoid flickering)
        if self.update_state.drawing_enabled() {
            self.handle_messages();
        }
        else {
            self.update_state.enable_draw();
        }
        self.draw_screen();
    }

    fn draw_screen(&mut self) {
        unsafe {
            let (width, height) = self.get_client_size();
            // used for handling maximize
            if width != self.bitmap_info.bmiHeader.biWidth || height != -self.bitmap_info.bmiHeader.biHeight {
                self.update_bitmap();
                // self.fill(self.background_color);
                self.update_state.cancel_draw();
            }
            if self.update_state.drawing_enabled() {
                StretchDIBits(
                    self.device_context,
                    0, // x
                    0, // y
                    width, // width
                    height, // height
                    0, // memory x
                    0, // memory y
                    width, // memory width
                    height, // memory height
                    self.video_memory_pointer,
                    &self.bitmap_info,
                    DIB_RGB_COLORS,
                    SRCCOPY
                );
            }
        }
    }

    pub fn draw_point(&mut self, index: usize, color: Color) {
        unsafe {
            *((self.video_memory_pointer as *mut u32).add(index)) = color.into();
        }
    }

    pub fn fill(&mut self, color: Color) {
        unsafe {
            let (width, height) = self.get_client_size();
            // println!("Fill size: ({}, {})", width, height);
            let color_u32: u32 = color.into();
            let mut offset: usize = 0;
            for _ in 0..width*height {
                *((self.video_memory_pointer as *mut u32).add(offset)) = color_u32;
                offset += 1;
            }
        }
        // println!("Finishes filling");
    }

    // window area excluding the taskbar
    pub fn get_client_size(&self) -> (i32, i32) {
        Window::get_client_size_from_handle(self.handle)
    }

    // size of the window including the taskbar
    pub fn get_window_size(&self) -> (i32, i32) {
        Window::get_window_size_from_handle(self.handle)
    }

    // position relative to the top left of the primary screen
    pub fn get_window_pos(&self) -> (i32, i32) {
        let client_rect = self.get_window_rect();
        (client_rect.left, client_rect.top)
    }

    pub fn get_window_rect(&self) -> RECT {
        Window::get_window_rect_from_handle(self.handle)
    }
    
    // relative to top left of window
    pub fn get_relative_cursor_pos(&self) -> (i32, i32) {
        let (gx, gy) = get_cursor_pos();
        let (wx, wy) = self.get_window_pos();
        (gx - wx, gy - wy)
    }

    pub fn get_taskbar_height(&self) -> i32 {
        Window::get_taskbar_height_from_handle(self.handle)
    }

    pub fn get_taskbar_height_from_handle(wind: HWND) -> i32 {
        let (_, window_height) = Window::get_window_size_from_handle(wind);
        let (_, client_height) = Window::get_client_size_from_handle(wind);
        window_height - client_height
    }

    pub fn get_window_size_from_handle(wind: HWND) -> (i32, i32) {
        let window_rect: RECT = Window::get_window_rect_from_handle(wind);
        (
            window_rect.right - window_rect.left,
            window_rect.bottom - window_rect.top
        )
    }

    pub fn get_client_size_from_handle(wind: HWND) -> (i32, i32) {
        let mut client_rect: RECT = Default::default();
        unsafe { GetClientRect(wind, &mut client_rect) };
        (
            client_rect.right - client_rect.left,
            client_rect.bottom - client_rect.top
        )
    }

    pub fn get_window_rect_from_handle(wind: HWND) -> RECT {
        let mut window_rect: RECT = Default::default();
        unsafe { GetWindowRect(wind, &mut window_rect) };
        window_rect
    }
}

// text in windows is in wide format
#[cfg(windows)]
fn win_32_string(text: &str) -> Vec<u16> {
    OsStr::new(text).encode_wide().chain(once(0)).collect()
}

// relative to top left of screen
#[cfg(windows)]
fn get_cursor_pos() -> (i32, i32) {
    let mut point = POINT{ x: 0, y: 0 };
    unsafe{ GetCursorPos(&mut point) };
    (point.x, point.y)
}

#[cfg(windows)]
fn generate_bitmap_info(width: i32, height: i32) -> BITMAPINFO {
    let mut bitmap_info: BITMAPINFO = Default::default();
    bitmap_info.bmiHeader = Default::default();
    bitmap_info.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
    bitmap_info.bmiHeader.biWidth = width;
    bitmap_info.bmiHeader.biHeight = -height;
    bitmap_info.bmiHeader.biPlanes = 1;
    bitmap_info.bmiHeader.biBitCount = 32;
    bitmap_info.bmiHeader.biCompression = BI_RGB;

    bitmap_info
}