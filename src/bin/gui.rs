use anyhow::Result;
use macroquad::color::BLACK;
use macroquad::prelude::{Conf, clear_background, next_frame};
use missile_sim::prelude::*;

fn _window_conf() -> Conf {
    Conf {
        window_title: String::from("Guidance Simulator"),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        sample_count: 1024,
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
