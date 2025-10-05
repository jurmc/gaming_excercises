use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

// https://mq.agical.se/ch4-falling-squares.html
// Falling Squares

const MOVEMENT_SPEED: f32 = 200.0;
const CIRCLE_R: f32 = 16.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

#[macroquad::main("Macroquad game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut colors = vec![
        RED,
        BLUE,
        GREEN,
        PINK,
        PURPLE,
        GRAY,
        WHITE,
        ORANGE,
        BROWN,
        GOLD,
        MAGENTA,
        LIME,
        BEIGE,
    ];
    colors.shuffle();

    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_R,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };


    loop {
        clear_background(DARKGRAY);
        let delta_time = get_frame_time();

        if rand::gen_range(0, 99) >=  95 {
            let size = rand::gen_range(10.0, 30.0);
            squares.push(Shape {
                size: size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: colors.choose().unwrap().clone(),
            });
        }

        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        squares.retain(|square| {
            square.y < screen_height() + square.size
        });

        for square in &squares {
            draw_rectangle(
                square.x + square.size / 2.0,
                square.y + square.size / 2.0,
                square.size,
                square.size,
                square.color);
        }

        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
        circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        next_frame().await;
    }
}
