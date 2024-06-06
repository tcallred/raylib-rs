// pub use raylib_ffi::colors;
// pub use raylib_ffi::enums;
// pub use raylib_ffi::Color;
// pub use raylib_ffi::{Camera, Camera2D, Camera3D};
// pub use raylib_ffi::{Vector2, Vector3, Vector4};
// TODO: Only re-export non ffi functions/types
pub use raylib_ffi::*;

pub mod rcore;
pub mod rshapes;
pub mod rtext;


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

