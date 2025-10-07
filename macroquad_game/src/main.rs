use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

// https://mq.agical.se/ch6-shooting.html
// Bullet Hell

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("Macroquad game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_R: f32 = 32.0;

    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_R,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };
    let mut gameover = false;

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

    loop {
        clear_background(DARKGRAY);

        if !gameover {
            let delta_time = get_frame_time();
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

            if rand::gen_range(0, 99) >=  95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: colors.choose().unwrap().clone(),
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            squares.retain(|square| square.y < screen_height() + square.size);
        }

        if squares.iter().any(|square| { circle.collides_with(&square) }) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color);
        }

        if gameover {
            let text = "GAME OVER";
            let text_dim = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dim.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED);
        }

        next_frame().await;
    }
}
