pub use raylib_ffi::colors;
pub use raylib_ffi::Color;
pub mod rcore {
    pub fn init_window(width: i32, height: i32, title: &str) {
        unsafe {
            raylib_ffi::InitWindow(width, height, raylib_ffi::rl_str!(title));
        }
    }

    pub fn window_should_close() -> bool {
        unsafe {
            raylib_ffi::WindowShouldClose()
        }
    }

    pub fn begin_drawing() {
        unsafe { raylib_ffi::BeginDrawing() }
    }

    pub fn end_drawing() {
        unsafe { raylib_ffi::EndDrawing() }
    }

    pub fn clear_background(color: raylib_ffi::Color) {
        unsafe {
            raylib_ffi::ClearBackground(color);
        }
    }

    pub fn close_window() {
        unsafe {
            raylib_ffi::CloseWindow();
        }
    }
}

pub mod rtext {
    use std::ffi::CString;
    pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: raylib_ffi::Color) {
        unsafe {
            let c_str_text = CString::new(text).expect("CString::new failed");
            raylib_ffi::DrawText(c_str_text.as_ptr(), pos_x, pos_y, font_size, color);
        }
    }
}
