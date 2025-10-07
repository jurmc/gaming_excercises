use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

// Points

fn reload(val: &mut f32, delta_time: f32) {
    const RELOAD_SPEED: f32 = 5.0;
    *val += RELOAD_SPEED * delta_time * 40.0;
    *val = clamp(*val, 0.0, 100.0);
    println!("Reloading: {}", val);
    draw_rectangle(
        0.0,
        screen_height() - 20.0,
        screen_width() * *val / 100.0,
        20.0,
        RED);
    draw_rectangle_lines(
        0.0,
        screen_height() - 20.0,
        screen_width(),
        20.0,
        2.0,
        ORANGE);
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
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
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_R,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        color: YELLOW,
    };
    let mut gameover = false;
    let mut reload_val = 100f32;

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
            if is_key_pressed(KeyCode::Space) {
                if reload_val > 99.9 {
                    reload_val = 0f32;
                    bullets.push(Shape {
                        size: 5.0,
                        speed: 2.0 * circle.speed,
                        x: circle.x,
                        y: circle.y,
                        collided: false,
                        color: RED,
                    })
                };
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

            if rand::gen_range(0, 99) >=  95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                    color: colors.choose().unwrap().clone(),
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size);

            reload(&mut reload_val, delta_time);
        }

        if squares.iter().any(|square| { circle.collides_with(&square) }) {
            gameover = true;
        }
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if square.collides_with(&bullet) {
                    square.collided = true;
                    bullet.collided = true;
                }
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
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
        for bullet in &bullets {
            draw_circle(
                bullet.x - bullet.size / 2.0,
                bullet.y - bullet.size / 2.0,
                bullet.size / 2.0,
                bullet.color);
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
