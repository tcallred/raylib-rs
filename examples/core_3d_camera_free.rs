use raylib::colors;
use raylib::enums;
use raylib::rcore;
use raylib::rcore::is_key_pressed;
use raylib::rmodels;
use raylib::Camera;
use raylib::Vector3;

pub fn main() {
    rcore::init_window(800, 450, "raylib-ffi example - camera free");
    let mut camera = Camera {
        position: Vector3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        },
        target: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        up: Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fovy: 45.0,
        projection: enums::CameraProjection::Perspective as i32,
    };

    let cube_position = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    rcore::disable_cursor();
    rcore::set_target_fps(60);
    while !rcore::window_should_close() {
        rcore::clear_background(colors::WHITE);
        rcore::update_camera(&mut camera, enums::CameraMode::Free);
        if is_key_pressed(enums::KeyboardKey::Z) {
            camera.target = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        rcore::begin_drawing();

        rcore::begin_mode_3d(camera);
        rmodels::draw_cube(cube_position, 2.0, 2.0, 2.0, colors::RED);
        rmodels::draw_cube_wires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);
        rmodels::draw_grid(10, 1.0);

        rcore::end_mode_3d();

        rcore::end_drawing();
    }

    rcore::close_window();
}
