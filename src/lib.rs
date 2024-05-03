pub use raylib_ffi::colors;
pub use raylib_ffi::enums;
pub use raylib_ffi::Color;
pub use raylib_ffi::{Camera, Camera2D, Camera3D};
pub use raylib_ffi::{Vector2, Vector3, Vector4};

pub mod rcore {
    // Window-related functions
    /// Initialize window and OpenGL context
    pub fn init_window(width: i32, height: i32, title: &str) {
        unsafe {
            raylib_ffi::InitWindow(width, height, raylib_ffi::rl_str!(title));
        }
    }

    /// Close window and unload OpenGL context
    pub fn close_window() {
        unsafe {
            raylib_ffi::CloseWindow();
        }
    }

    /// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
    pub fn window_should_close() -> bool {
        unsafe { raylib_ffi::WindowShouldClose() }
    }

    // Cursor-related functions
    /// Disables cursor (lock cursor)
    pub fn disable_cursor() {
        unsafe { raylib_ffi::DisableCursor() }
    }

    /// Drawing-related functions
    pub fn clear_background(color: raylib_ffi::Color) {
        unsafe {
            raylib_ffi::ClearBackground(color);
        }
    }

    /// Setup canvas (framebuffer) to start drawing
    pub fn begin_drawing() {
        unsafe { raylib_ffi::BeginDrawing() }
    }

    /// End canvas drawing and swap buffers (double buffering)
    pub fn end_drawing() {
        unsafe { raylib_ffi::EndDrawing() }
    }

    /// Begin 3D mode with custom camera (3D)
    pub fn begin_mode_3d(camera: raylib_ffi::Camera3D) {
        unsafe { raylib_ffi::BeginMode3D(camera) }
    }

    /// Ends 3D mode and returns to default 2D orthographic mode
    pub fn end_mode_3d() {
        unsafe { raylib_ffi::EndMode3D() }
    }

    // Timing-related functions
    /// Set target FPS (maximum)
    pub fn set_target_fps(fps: i32) {
        unsafe { raylib_ffi::SetTargetFPS(fps) }
    }

    // Input-related functions: keyboard
    /// Check if a key has been pressed once
    pub fn is_key_pressed(key: raylib_ffi::enums::KeyboardKey) -> bool {
        unsafe { raylib_ffi::IsKeyPressed(key as i32) }
    }

    // Camera System Functions
    /// Update camera position for selected mode
    pub fn update_camera(camera: &mut raylib_ffi::Camera, mode: raylib_ffi::enums::CameraMode) {
        unsafe { raylib_ffi::UpdateCamera(camera, mode as i32) }
    }
}

pub mod rtext {
    use std::ffi::CString;
    // Text drawing functions
    /// Draw text (using default font)
    pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: raylib_ffi::Color) {
        unsafe {
            let c_str_text = CString::new(text).expect("CString::new failed");
            raylib_ffi::DrawText(c_str_text.as_ptr(), pos_x, pos_y, font_size, color);
        }
    }
}

pub mod rmodels {
    use raylib_ffi::Color;
    use raylib_ffi::Vector3;
    // Basic geometric 3D shapes drawing functions
    /// Draw cube
    pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { raylib_ffi::DrawCube(position, width, height, length, color) }
    }

    /// Draw cube wires
    pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { raylib_ffi::DrawCubeWires(position, width, height, length, color) }
    }

    /// Draw a grid (centered at (0, 0, 0))
    pub fn draw_grid(slices: i32, spacing: f32) {
        unsafe { raylib_ffi::DrawGrid(slices, spacing) }
    }
}

pub mod rshapes {
    use raylib_ffi::Color;

    // Basic shapes drawing functions
    /// Draw a color-filled rectangle
    pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        unsafe { raylib_ffi::DrawRectangle(pos_x, pos_y, width, height, color) }
    }

    /// Draw rectangle outline
    pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        unsafe { raylib_ffi::DrawRectangleLines(pos_x, pos_y, width, height, color) }
    }
}
