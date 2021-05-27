#![allow(dead_code)]

// prevent console from opening
#![windows_subsystem = "windows"]

// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/libloaderapi/index.html?search=winuser
extern crate winapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::{null_mut};
use std::time::{Instant, Duration};

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
    GetAsyncKeyState,
    ShowWindow,
    IsIconic,
    GetActiveWindow,
    SetTimer,
    KillTimer,
    GET_WHEEL_DELTA_WPARAM,
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
    WS_MAXIMIZEBOX,
    WS_SIZEBOX,
    SW_SHOWMAXIMIZED,
    SW_SHOWNORMAL,
    SC_RESTORE,
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
    WM_MBUTTONDOWN,
    WM_MBUTTONUP,
    WM_MOUSEWHEEL,
    WM_MOUSEHWHEEL,
    WM_XBUTTONDOWN,
    WM_XBUTTONUP,
    WM_KEYDOWN,
    WM_KEYUP,

    WM_NCLBUTTONDOWN,
    WM_NCLBUTTONUP,
    WM_NCMOUSEMOVE,
    WM_NCMOUSELEAVE,
    WM_SYSCOMMAND,

    WM_ENTERSIZEMOVE,
    WM_EXITSIZEMOVE,
    WM_NCHITTEST,

    SWP_DRAWFRAME,
    SWP_NOOWNERZORDER,

    TME_LEAVE,
    TME_NONCLIENT,

    GET_XBUTTON_WPARAM,
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
    HTCLOSE,
    HTMAXBUTTON,
    HTMINBUTTON,
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
    WPARAM,
    LRESULT,
    UINT
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
use self::winapi::shared::windowsx::{
    GET_X_LPARAM,
    GET_Y_LPARAM,
};

use std::os::raw::c_int;
use std::io::Error;

// export color module
pub mod color;
use color::Color;

// export event module
pub mod event;
use event::{EventManager, WindowEvent, MouseEvent, MouseButton};

static mut WINDOWCOUNT: u32 = 0;

static mut WINDOWPTR: *mut Window = null_mut();

const MODAL_TIMER_ID: usize = 0;

type MoveCallback = fn(&mut Window) -> Result<(), &'static str>;

//#region WindowBuilder
pub struct WindowBuilder{
    title: Vec<u16>,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    background_color: Color,
    show_frame_rate: bool,
    min_size: (i32, i32),
    max_size: (i32, i32),
    start_maximized: bool,
    allow_resize: bool,
    allow_maximize: bool,
    close_approval: bool
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            title: win_32_string("New Window"),
            background_color: Color::WHITE,
            show_frame_rate: false,
            min_size: (-1, -1),
            max_size: (-1, -1),
            start_maximized: false,
            allow_resize: true,
            allow_maximize: true,
            close_approval: false,
        }
    }

    pub fn set_x(mut self, x: i32) -> Self {
        self.ref_set_x(x);
        self
    }

    pub fn ref_set_x(&mut self, x: i32) -> &mut Self {
        self.x = x as c_int;
        self
    }

    pub fn set_y(mut self, y: i32) -> Self {
        self.ref_set_y(y);
        self
    }

    pub fn ref_set_y(&mut self, y: i32) -> &mut Self {
        self.y = y as c_int;
        self
    }

    pub fn set_position(mut self, x: i32, y: i32) -> Self {
        self.ref_set_position(x, y);
        self
    }

    pub fn ref_set_position(&mut self, x: i32, y: i32) -> &mut Self {
        self.ref_set_x(x);
        self.ref_set_y(y);
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.ref_set_width(width);
        self
    }

    pub fn ref_set_width(&mut self, width: i32) -> &mut Self {
        self.width = width as c_int;
        self
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.ref_set_height(height);
        self
    }

    pub fn ref_set_height(&mut self, height: i32) -> &mut Self {
        self.height = height as c_int;
        self
    }

    pub fn set_size(mut self, width: i32, height: i32) -> Self {
        self.ref_set_size(width, height);
        self
    }

    pub fn ref_set_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.ref_set_width(width);
        self.ref_set_height(height);
        self
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.ref_set_title(title);
        self
    }

    pub fn ref_set_title(&mut self, title: &str) -> &mut Self {
        self.title = win_32_string(title);
        self
    }

    pub fn set_background_color(mut self, color: Color) -> Self {
        self.ref_set_background_color(color);
        self
    }

    pub fn ref_set_background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    pub fn show_frame_rate(mut self) -> Self {
        self.ref_show_frame_rate();
        self
    }

    pub fn ref_show_frame_rate(&mut self) -> &mut Self {
        self.show_frame_rate = true;
        self
    }

    pub fn hide_frame_rate(mut self) -> Self {
        self.ref_hide_frame_rate();
        self
    }

    pub fn ref_hide_frame_rate(&mut self) -> &mut Self {
        self.show_frame_rate = false;
        self
    }

    pub fn start_maximized(self) -> Self {
        self.set_start_maximized(true)
    }

    pub fn ref_start_maximixed(&mut self) -> &mut Self {
        self.ref_set_start_maximized(true)
    }

    pub fn start_unmaximixed(self) -> Self {
        self.set_start_maximized(false)
    }
    
    pub fn ref_start_unmaximized(&mut self) -> &mut Self {
        self.ref_set_start_maximized(false)
    }

    pub fn set_start_maximized(mut self, start_maximized: bool) -> Self {
        self.ref_set_start_maximized(start_maximized);
        self
    }

    pub fn ref_set_start_maximized(&mut self, start_maximized: bool) -> &mut Self {
        if self.allow_maximize {
            self.start_maximized = start_maximized;
        }
        else {
            self.start_maximized = false;
        }
        self
    }

    pub fn set_min_size(mut self, width: i32, height: i32) -> Self {
        self.ref_set_min_size(width, height);
        self
    }

    pub fn ref_set_min_size(&mut self, width: i32, height: i32) -> &mut Self {
        if self.max_size > (width, height) {
            self.min_size = (width, height);
        }
        else if self.max_size.0 == -1 || self.max_size.1 == -1 {
            if self.max_size.0 == -1 || self.max_size.0 > width {
                self.min_size = (width, self.min_size.1);
            }
            if self.max_size.1 == -1 || self.max_size.1 > height {
                self.min_size = (self.min_size.0, height);
            }
        }
        self
    }

    pub fn set_max_size(mut self, width: i32, height: i32) -> Self {
        self.ref_set_max_size(width, height);
        self
    }

    pub fn ref_set_max_size(&mut self, width: i32, height: i32) -> &mut Self {
        if self.min_size < (width, height) {
            self.max_size = (width, height);
        }
        else if self.min_size.0 == -1 || self.min_size.1 == -1 {
            if self.min_size.0 == -1 || self.min_size.0 < width {
                self.max_size = (width, self.max_size.1);
            }
            if self.min_size.1 == -1 || self.min_size.1 < height {
                self.max_size = (self.max_size.0, height);
            }
        }
        self
    }

    pub fn allow_resize(self) -> Self {
        self.set_allow_resize(true)
    }

    pub fn ref_allow_resize(&mut self) -> &mut Self {
        self.ref_set_allow_resize(true)
    }

    pub fn disable_resize(self) -> Self {
        self.set_allow_resize(false)
    }

    pub fn ref_disable_resize(&mut self) -> &mut Self {
        self.ref_set_allow_resize(false)
    }

    pub fn set_allow_resize(mut self, allow: bool) -> Self {
        self.ref_set_allow_resize(allow);
        self
    }

    pub fn ref_set_allow_resize(&mut self, allow: bool) -> &mut Self {
        self.allow_resize = allow;
        self
    }

    pub fn allow_maximize(self) -> Self {
        self.set_allow_maximize(true)
    }

    pub fn ref_allow_maximize(&mut self) -> &mut Self {
        self.ref_set_allow_maximize(true)
    }

    pub fn disable_maximize(self) -> Self {
        self.set_allow_maximize(false)
    }

    pub fn ref_disable_maximize(&mut self) -> &mut Self {
        self.ref_set_allow_maximize(false)
    }

    pub fn set_allow_maximize(mut self, allow: bool) -> Self {
        self.ref_set_allow_maximize(allow);
        self
    }

    pub fn ref_set_allow_maximize(&mut self, allow: bool) -> &mut Self {
        self.allow_maximize = allow;
        if !self.allow_maximize {
            self.start_maximized = false;
        }
        self
    }

    pub fn normal_close_mode(mut self) -> Self {
        self.ref_normal_close_mode();
        self
    }

    pub fn ref_normal_close_mode(&mut self) -> &mut Self {
        self.close_approval = false;
        self
    }

    pub fn approve_close_mode(mut self) -> Self {
        self.ref_approve_close_mode();
        self
    }

    pub fn ref_approve_close_mode(&mut self) -> &mut Self {
        self.close_approval = true;
        self
    }

    pub fn set_close_mode(mut self, approve: bool) -> Self {
        self.ref_set_close_mode(approve);
        self
    }

    pub fn ref_set_close_mode(&mut self, approve: bool) -> &mut Self {
        self.close_approval = approve;
        self
    }

    pub fn build(mut self) -> Window {
        self.ref_build()
    }

    pub fn ref_build(&mut self) -> Window {
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
                lpfnWndProc: Some(window_proc),
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

            let mut window_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

            if !self.allow_resize {
                window_style &= !WS_SIZEBOX;
            }

            if !self.allow_maximize {
                window_style &= !WS_MAXIMIZEBOX;
            }

            if self.start_maximized {
                window_style |= WS_MAXIMIZEBOX; 
            }

            // create a display window from the registered window class
            // https://msdn.microsoft.com/en-us/library/windows/desktop/ms632680(v=vs.85).aspx
            let handle = CreateWindowExW(
                0,
                class_name.as_ptr(),
                self.title.as_ptr(),
                window_style,
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

            if self.start_maximized {
                let result = ShowWindow(handle, SW_SHOWMAXIMIZED);
                if result == 0 {
                    println!("{}", Error::last_os_error());
                }
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

            // ensure the minimum size the window can be is the taskbar height
            if self.min_size.1 < Window::get_taskbar_height_from_handle(handle) {
                self.min_size = (self.min_size.0, Window::get_taskbar_height_from_handle(handle) + 1);
            }

            Window {
                handle,
                device_context: GetDC(handle),
                video_memory_pointer,
                bitmap_info: generate_bitmap_info(client_width, client_height),
                minimum_size: self.min_size,
                maximum_size: self.max_size,
                background_color: self.background_color,
                update_state: UpdateState::new(handle),
                show_frame_rate: self.show_frame_rate,
                frame_count: 0,
                frame_start_time: None,
                event_manager: EventManager::new(),
                maximized: self.start_maximized,
                close_mode: if self.close_approval {
                    CloseState::REQUIREAPPROVE
                }
                else {
                    CloseState::NORMAL
                },
                focused: true,
                window_move_callback: |_|Ok(()),
                modal_running: false,
                timer_callback_running: false
            }
        }
    }
}

impl std::fmt::Debug for WindowBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowBuilder")
            .field("Title", &String::from_utf16(&self.title))
            .field("X Position", &self.x)
            .field("Y Position", &self.y)
            .field("Width", &self.width)
            .field("Height", &self.height)
            .field("Background Color", &self.background_color)
            .field("Min Size", &self.min_size)
            .field("Max Size", &self.max_size)
            .field("Show FPS", &self.show_frame_rate)
            .field("Start Maximized", &self.start_maximized)
            .field("Allow Maximize", &self.allow_maximize)
            .field("Allow Resize", &self.allow_resize)
            .finish()
    }
}
//#endregion

//#region UpdateState & CloseState
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

pub enum CloseState {
    NORMAL,
    REQUIREAPPROVE,
    AWAITINGAPPROVE
}
//#endregion

//#region Window
pub struct Window {
    handle: HWND,
    device_context: HDC,
    video_memory_pointer: *mut VOID,
    bitmap_info: BITMAPINFO,
    minimum_size: (i32, i32),
    maximum_size: (i32, i32),
    background_color: Color,
    update_state: UpdateState,
    show_frame_rate: bool,
    frame_count: i32,
    frame_start_time: Option<Instant>,
    event_manager: EventManager,
    maximized: bool,
    close_mode: CloseState,
    focused: bool,
    window_move_callback: MoveCallback,
    modal_running: bool,
    timer_callback_running: bool
}

impl Window {
    fn is_resizing(&mut self) -> bool {
        if self.left_mouse_down() && self.update_state.get_sizing_direction() != 0 {
            true
        }
        else {
            self.update_state.clear_sizing_direction();
            false
        }
    }

    fn left_mouse_down(&self) -> bool {
        (unsafe{ GetAsyncKeyState(VK_LBUTTON) }) as u16 & 0x8000 == 0x8000
    }

    fn defer_window(&mut self, x: i32, y: i32, width: i32, height: i32, flags: UINT) {
        #[cfg(feature="window_profile")]
        let defer_timer = Instant::now();
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
        #[cfg(feature="window_profile")]
        println!("\tDefer Window Time: {}ms", defer_timer.elapsed().as_millis());
    }

    fn update_bitmap(&mut self) {
        #[cfg(feature="window_profile")]
        let bitmap_timer = Instant::now();
        let (client_width, client_height) = self.get_client_size();
        unsafe {
            // ensure the memory from the last section of video memory is freed
            if self.video_memory_pointer != null_mut() {
                let free_result = VirtualFree(
                    self.video_memory_pointer,
                    0,
                    MEM_RELEASE
                );
                if free_result == 0 {
                    panic!("{}", Error::last_os_error());
                }
            }
            self.video_memory_pointer = VirtualAlloc(
                null_mut(),
                (client_width * client_height * 4) as SIZE_T, // * 4 due to there being 4 bytes per pixel
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE
            );
            if self.video_memory_pointer.is_null() {
                panic!("{}", Error::last_os_error());
            }
            self.bitmap_info = generate_bitmap_info(client_width, client_height);
            self.fill(self.background_color);
        }
        #[cfg(feature="window_profile")]
        println!("\tUpdate Bitmap Timer: {}ms", bitmap_timer.elapsed().as_millis());
    }

    fn clamp_width(&self, width: i32) -> i32 {
        if width > self.maximum_size.0 && self.maximum_size.0 != -1 {
            self.maximum_size.0
        }
        else if width < self.minimum_size.0 && self.minimum_size.0 != -1 {
            self.minimum_size.0
        }
        else {
            width
        }
    }

    fn clamp_height(&self, height: i32) -> i32 {
        if height > self.maximum_size.1 && self.maximum_size.1 != -1 {
            self.maximum_size.1
        }
        else if height < self.minimum_size.1 && self.minimum_size.1 != -1 {
            self.minimum_size.1
        }
        else {
            height
        }
    }

    fn handle_resize(&mut self) {
        #[cfg(feature="window_profile")]
        let resize_timer = Instant::now();
        let (cursor_x, cursor_y) = get_cursor_pos();
        // ensure the cursor has moved
        if self.update_state.get_cached_cursor_pos() != (cursor_x, cursor_y) {
            let window_rect = self.get_window_rect();
            let (mut dx, mut dy) = (0, 0);
            let (dwidth, dheight) = match self.update_state.get_sizing_direction() {
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
            self.defer_window(
                window_rect.left - dx,
                window_rect.top - dy,
                self.clamp_width(width + dwidth),
                self.clamp_height(height + dheight),
                SWP_DRAWFRAME | SWP_NOOWNERZORDER
            );
            self.event_manager.push_event(WindowEvent::WINDOWRESIZE);
            self.update_state.cache_cursor_pos((cursor_x, cursor_y));
            self.update_bitmap();
            self.update_state.cancel_draw();
        }
        #[cfg(feature="window_profile")]
        println!("\tResize Timer: {}ms", resize_timer.elapsed().as_millis());
    }

    pub fn handle_messages(&mut self) {
        // ensure the WINDOWPTR points correctly to this window before processing messages
        unsafe { WINDOWPTR = self as *mut Window; }
        #[cfg(feature="window_profile")]
        let message_timer = Instant::now();
        unsafe {
            // only track the cursor if the window is being resized
            if self.is_resizing() {
                self.update_state.track_mouse();
            }
            let message = mem::MaybeUninit::<MSG>::uninit();
            while PeekMessageW(message.as_ptr() as *mut MSG, self.handle, 0, 0, PM_REMOVE) != 0 {
                let message_code = (*(message.as_ptr())).message;
                let l_param = (*(message.as_ptr())).lParam;
                let w_param = (*(message.as_ptr())).wParam;
                let mut use_default_protocol = true;
                match message_code {
                    // client area events
                    WM_MOUSEMOVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                            use_default_protocol = false;
                        }
                        else {
                            self.event_manager.push_event(MouseEvent::MOUSEMOVE);
                        }
                    },
                    WM_MOUSELEAVE => {
                        if self.is_resizing() {
                            self.handle_resize();
                        }
                    },

                    WM_LBUTTONDOWN => self.event_manager.push_event(
                        MouseEvent::MOUSEDOWN(
                            MouseButton::LEFTMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),
                    WM_LBUTTONUP => self.event_manager.push_event(
                        MouseEvent::MOUSEUP(
                            MouseButton::LEFTMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),

                    WM_RBUTTONDOWN => self.event_manager.push_event(
                        MouseEvent::MOUSEDOWN(
                            MouseButton::RIGHTMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),
                    WM_RBUTTONUP => self.event_manager.push_event(
                        MouseEvent::MOUSEUP(
                            MouseButton::RIGHTMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),

                    WM_MBUTTONDOWN => self.event_manager.push_event(
                        MouseEvent::MOUSEDOWN(
                            MouseButton::MIDDLEMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),
                    WM_MBUTTONUP => self.event_manager.push_event(
                        MouseEvent::MOUSEUP(
                            MouseButton::MIDDLEMOUSE,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),

                    WM_XBUTTONDOWN => {
                        let button = if GET_XBUTTON_WPARAM(w_param) == 1 {
                            MouseButton::XBUTTON
                        }
                        else {
                            MouseButton::YBUTTON
                        };
                        self.event_manager.push_event(
                            MouseEvent::MOUSEDOWN(
                                button,
                                GET_X_LPARAM(l_param),
                                GET_Y_LPARAM(l_param)
                            )
                        );
                    },
                    WM_XBUTTONUP => {
                        let button = if GET_XBUTTON_WPARAM(w_param) == 1 {
                            MouseButton::XBUTTON
                        }
                        else {
                            MouseButton::YBUTTON
                        };
                        self.event_manager.push_event(
                            MouseEvent::MOUSEDOWN(
                                button,
                                GET_X_LPARAM(l_param),
                                GET_Y_LPARAM(l_param)
                            )
                        );
                    },

                    WM_MOUSEWHEEL => self.event_manager.push_event(
                        MouseEvent::MOUSESCROLL(
                            0,
                            GET_WHEEL_DELTA_WPARAM(w_param),
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),
                    WM_MOUSEHWHEEL => self.event_manager.push_event(
                        MouseEvent::MOUSESCROLL(
                            GET_WHEEL_DELTA_WPARAM(w_param),
                            0,
                            GET_X_LPARAM(l_param),
                            GET_Y_LPARAM(l_param)
                        )
                    ),

                    WM_KEYDOWN => self.event_manager.register_key_down(w_param),
                    WM_KEYUP => self.event_manager.register_key_up(w_param),

                    // nc events (taskbar, resizing, syscommand etc)
                    WM_NCLBUTTONDOWN => {
                        match w_param as isize {
                            HTTOPLEFT | HTTOPRIGHT |
                            HTBOTTOMLEFT | HTBOTTOMRIGHT |
                            HTTOP | HTRIGHT |
                            HTBOTTOM | HTLEFT => {
                                self.update_state.set_sizing_direction(w_param as LPARAM);
                                self.update_state.cache_cursor_pos(get_cursor_pos());
                                use_default_protocol = false;
                            },
                            HTCLOSE => {
                                #[allow(unreachable_patterns)]
                                match self.close_mode {
                                    CloseState::REQUIREAPPROVE => {
                                        self.event_manager.push_event(WindowEvent::WINDOWCLOSEBEGIN);
                                        use_default_protocol = false;
                                        unimplemented!("Close approval needs implementing");
                                    },
                                    CloseState::AWAITINGAPPROVE | CloseState::NORMAL | _ => {
                                        self.event_manager.push_event(WindowEvent::WINDOWCLOSEFINAL);
                                    }
                                }
                            },
                            HTMAXBUTTON => {
                                self.maximized = !self.maximized;
                                ShowWindow(
                                    self.handle,
                                    if self.maximized {
                                        SW_SHOWMAXIMIZED
                                    }
                                    else {
                                        SW_SHOWNORMAL
                                    }
                                );
                                self.event_manager.push_event(
                                    if self.maximized {
                                        WindowEvent::WINDOWMAXIMIZE
                                    }
                                    else {
                                        WindowEvent::WINDOWUNMAXIMIZE
                                    }
                                );
                            },
                            HTMINBUTTON => self.event_manager.push_event(WindowEvent::WINDOWMINIMIZE),
                            _ => ()
                        }
                    },
                    WM_NCLBUTTONUP => {
                        KillTimer(self.handle, MODAL_TIMER_ID);
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
                        if w_param == SC_RESTORE {
                            self.event_manager.push_event(WindowEvent::WINDOWRESTORE);
                        }
                    },
                    
                    _ => {
                        println!("Uncaught: {}", (*(message.as_ptr())).message);
                    }
                }
                if use_default_protocol {
                    TranslateMessage(message.as_ptr() as *const MSG);
                    DispatchMessageW(message.as_ptr() as *const MSG);
                }
            }
        }

        #[cfg(feature="window_profile")]
        println!("\tMessage Time: {}ms", message_timer.elapsed().as_millis());
    }

    // draws the window and handles any messages
    pub fn update(&mut self) {
        if !self.timer_callback_running {
            self.modal_running = false;
        }
        #[cfg(feature="window_profile")]
        let window_update_timer = Instant::now();
        #[cfg(feature="window_profile")]
        println!("Window Update:\t");
        let window_is_focused = unsafe { GetActiveWindow() } == self.handle;
        if window_is_focused && !self.focused {
            self.focused = true;
            self.event_manager.push_event(WindowEvent::WINDOWFOCUS);
        }
        else if !window_is_focused && self.focused {
            self.focused = false;
            self.event_manager.push_event(WindowEvent::WINDOWBLUR);
        }
        // ensure the screen is drawn at least every other frame without interference
        // of windows messages (used to avoid flickering)
        if self.update_state.drawing_enabled() {
            self.handle_messages();
        }
        else {
            self.update_state.enable_draw();
        }
        if self.is_running() {
            self.draw_screen();
            if self.show_frame_rate || self.frame_start_time.is_some() {
                if self.frame_start_time.is_none() {
                    self.frame_start_time = Some(Instant::now());
                }
                const SECONDDURATION: Duration = Duration::from_secs(1);
                self.frame_count += 1;
                if self.frame_start_time.unwrap().elapsed() >= SECONDDURATION {
                    println!("Frames elapsed: {}", self.frame_count);
                    self.frame_count = 0;
                    if self.show_frame_rate {
                        self.frame_start_time = Some(Instant::now());
                    }
                    else {
                        self.frame_start_time = None;
                    }
                }
            }
        }
        #[cfg(feature="window_profile")]
        println!("\tWindow Update Time: {}ms\nEnd Window Update", window_update_timer.elapsed().as_millis());
    }

    fn draw_screen(&mut self) {
        if self.is_minimized() {
            return;
        }
        #[cfg(feature="window_profile")]
        let screen_draw_timer = Instant::now();
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
        #[cfg(feature="window_profile")]
        println!("\tScreen Draw Time: {}ms", screen_draw_timer.elapsed().as_millis());
    }

    fn blip(&mut self, index: usize, color: Color) {
        unsafe {
            *((self.video_memory_pointer as *mut u32).add(index)) = color.into();
        }
    }

    pub fn draw_point(&mut self, x: i32, y: i32, color: Color) {
        let (width, height) = self.get_client_size();
        let x_filtered = if x < 0 {
            0
        }
        else if x >= width {
            width - 1
        }
        else {
            x
        };
        let y_filtered = if y < 0 {
            0
        }
        else if y >= height {
            height - 1
        }
        else {
            y
        };
        let index = y_filtered * width + x_filtered;
        self.blip(index as usize, color);
    }
    
    // uses Bresenham’s Line Generation Algorithm
    pub fn draw_line(&mut self, point_a: (i32, i32), point_b: (i32, i32), color: Color) {
        let mut dx = point_b.0 - point_a.0;
        let mut dy = point_b.1 - point_a.1;
        let step_x = if dx < 0 {
            dx = -dx;
            -1
        }
        else{
            1
        };
        let step_y = if dy < 0 {
            dy = -dy;
            -1
        }
        else {
            1
        };
        // double dx and dy
        dy <<= 1;
        dx <<= 1;
        let mut x = point_a.0;
        let mut y = point_a.1;
        self.draw_point(x, y, color);
        if dx > dy {
            let mut fraction = dy - (dx >> 1);
            while x != point_b.0 {
                x += step_x;
                if fraction >= 0 {
                    y += step_y;
                    fraction -= dx;
                }
                fraction += dy;
                self.draw_point(x, y, color);
            }
        }
        else {
            let mut fraction = dx - (dy >> 2);
            while y != point_b.1 {
                y += step_y;
                if fraction >= 0 {
                    x += step_x;
                    fraction -= dy;
                }
                fraction += dx;
                self.draw_point(x, y, color);
            }
        }
    }

    pub fn fill(&mut self, color: Color) {
        #[cfg(feature="window_profile")]
        let fill_timer = Instant::now();
        unsafe {
            let (width, height) = self.get_client_size();
            let color_u32: u32 = color.into();
            let mut offset: usize = 0;
            for _ in 0..width*height {
                *((self.video_memory_pointer as *mut u32).add(offset)) = color_u32;
                offset += 1;
            }
        }
        #[cfg(feature="window_profile")]
        println!("\tFill Time: {}ms", fill_timer.elapsed().as_millis());
    }

    pub fn show_frame_rate(&mut self) {
        self.show_frame_rate = true;
    }

    pub fn hide_frame_rate(&mut self) {
        self.show_frame_rate = false;
    }

    pub fn set_frame_rate_display(&mut self, show: bool) {
        self.show_frame_rate = show;
    }

    pub fn is_running(&self) -> bool {
        unsafe { IsWindow(self.handle) != 0 }
    }

    pub fn is_minimized(&self) -> bool {
        (unsafe { IsIconic(self.handle) }) != 0
    }

    pub fn get_background_color(&self) -> Color {
        self.background_color
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
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
        Self::get_taskbar_height_from_handle(self.handle)
    }

    pub fn get_taskbar_height_from_handle(wind: HWND) -> i32 {
        let (_, window_height) = Self::get_window_size_from_handle(wind);
        let (_, client_height) = Self::get_client_size_from_handle(wind);
        window_height - client_height
    }

    pub fn get_window_size_from_handle(wind: HWND) -> (i32, i32) {
        let window_rect: RECT = Self::get_window_rect_from_handle(wind);
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

    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }

    pub fn event_manager_mut(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }

    pub fn set_move_callback(&mut self, callback: MoveCallback) {
        self.window_move_callback = callback;
    }
}
//#endregion

// text in windows is in wide format
fn win_32_string(text: &str) -> Vec<u16> {
    OsStr::new(text).encode_wide().chain(once(0)).collect()
}

// relative to top left of screen
fn get_cursor_pos() -> (i32, i32) {
    let mut point = POINT{ x: 0, y: 0 };
    unsafe{ GetCursorPos(&mut point) };
    (point.x, point.y)
}

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

unsafe extern "system"
fn window_proc(handle: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        WM_NCLBUTTONDOWN => {
            println!("Timer started");
            set_modal_running();
            SetTimer(handle, MODAL_TIMER_ID, 1, Some(modal_timer_proc));
            DefWindowProcW(handle, message, w_param, l_param)
        },
        WM_NCLBUTTONUP => {
            println!("Timer ended");
            KillTimer(handle, MODAL_TIMER_ID);
            DefWindowProcW(handle, message, w_param, l_param)
        },
        WM_ENTERSIZEMOVE => {
            println!("Enter size move");
            set_modal_running();
            SetTimer(handle, MODAL_TIMER_ID, 1, Some(modal_timer_proc));
            DefWindowProcW(handle, message, w_param, l_param)
        },
        WM_EXITSIZEMOVE => {
            println!("Enter size move");
            KillTimer(handle, MODAL_TIMER_ID);
            DefWindowProcW(handle, message, w_param, l_param)
        },
        WM_NCHITTEST => {
            println!("Hit test");
            DefWindowProcW(handle, message, w_param, l_param)
        },

        _ => DefWindowProcW(handle, message, w_param, l_param)
    }
}

unsafe extern "system"
fn modal_timer_proc(handle: HWND, _: UINT, _: usize, _: DWORD) {
    println!("Timer proc");
    if WINDOWPTR != null_mut() {
        let window = &mut *WINDOWPTR;
        if window.modal_running {
            window.timer_callback_running = true;
            (window.window_move_callback)(window).unwrap();
            window.timer_callback_running = false;
        }
        else {
            KillTimer(handle, MODAL_TIMER_ID);
        }
    }
}

unsafe
fn set_modal_running() {
    if WINDOWPTR != null_mut() {
        let window = &mut *WINDOWPTR;
        window.modal_running = true;
    }
}