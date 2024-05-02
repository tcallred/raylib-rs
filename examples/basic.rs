use raylib::rcore;
use raylib::rtext;
pub fn main() {
    rcore::init_window(800, 450, "raylib-rs example - basic window");

    while !rcore::window_should_close() {
        rcore::begin_drawing();
        rcore::clear_background(raylib::colors::WHITE);
        rtext::draw_text(
            "Congrats! you created your first window!",
            190,
            200,
            20,
            raylib::colors::BLACK,
        );
        rcore::end_drawing();
    }

    rcore::close_window();
}
