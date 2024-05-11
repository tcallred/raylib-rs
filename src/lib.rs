// pub use raylib_ffi::colors;
// pub use raylib_ffi::enums;
// pub use raylib_ffi::Color;
// pub use raylib_ffi::{Camera, Camera2D, Camera3D};
// pub use raylib_ffi::{Vector2, Vector3, Vector4};
// TODO: Only re-export non ffi functions/types
pub use raylib_ffi::*;

pub mod rcore;
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
