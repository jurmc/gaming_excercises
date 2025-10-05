use macroquad::prelude::*;

#[macroquad::main("Macroquad game")]
async fn main() {
    loop {
        clear_background(DARKGRAY);
        next_frame().await;
    }
}
