/// A debug binary for testing and experimenting with the missile simulation engine.
/// //TODO: complete it
use missile_sim::prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Raylib").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("Hello, Raylib!", 190, 200, 20, Color::DARKGRAY);
    }
}
