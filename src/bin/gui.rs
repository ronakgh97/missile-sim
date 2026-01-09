use anyhow::Result;
use macroquad::color::BLACK;
use macroquad::prelude::{Conf, clear_background, next_frame};
use missile_sim::prelude::*;

fn _window_conf() -> Conf {
    Conf {
        window_title: String::from("Guidance Simulator"),
        window_width: 4 * 256,
        window_height: 3 * 256,
        window_resizable: true,
        sample_count: 4096,
        ..Default::default()
    }
}

#[macroquad::main(_window_conf)]
async fn main() -> Result<()> {
    loop {
        clear_background(BLACK);

        render_master().await?;

        // Update the screen
        next_frame().await;
    }
}
