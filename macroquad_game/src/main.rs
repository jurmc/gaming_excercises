use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad_particles::{self as particles, AtlasConfig, Emitter, EmitterConfig};
use macroquad::experimental::animation::{AnimatedSprite, Animation};

use std::fs;

// Graphics, Graphical Explosions

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");
const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
    }
";

fn draw_help() {
    let mut y = 25.0;
    draw_text("Help:", 20.0, y, 25.0, GRAY);
    y += 25.0;
    draw_text(" Escape - exit game", 20.0, y, 25.0, GRAY);
    y += 25.0;
    draw_text(" Space - start/resume game", 20.0, y, 25.0, GRAY);
}

fn reload(val: &mut f32, delta_time: f32) {
    const RELOAD_SPEED: f32 = 5.0;
    *val += RELOAD_SPEED * delta_time;
    *val = clamp(*val, 0.0, 100.0);
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

fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.9,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 400.0,
        initial_velocity_randomness: 0.8,
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5, 1, 0..)),
        ..Default::default()
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
    color: Color,
}

enum GameState {
    MainMenu,
    Started,
    Paused,
    GameOver,

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

    const GEN_FREQ: f64 = 4.0 * 20.0;
    const GEN_TIME_CNT_MAX: f64 = 1.0 / GEN_FREQ;
    let mut gen_time_cnt = 0.0;

    set_pc_assets_folder("assets");
    let ship_texture = load_texture("ship.png").await.expect("Could not load file");
    ship_texture.set_filter(FilterMode::Nearest);
    let bullet_texture = load_texture("laser-bolts.png").await.expect("Could not load file");
    bullet_texture.set_filter(FilterMode::Nearest);
    let explosion_texture = load_texture("explosion.png").await.expect("Could not load file");
    explosion_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let mut bullet_sprite = AnimatedSprite::new(
        16, 16,
        &[
            Animation {
                name: "bullet".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "bolt".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );

    let mut ship_sprite = AnimatedSprite::new(
        16, 24,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "left".to_string(),
                row: 2,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "right".to_string(),
                row: 4,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );

    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];
    let mut circle = Shape {
        size: CIRCLE_R,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        color: YELLOW,
    };
    let mut reload_val = 100f32;
    let mut state = GameState::MainMenu;

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

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

    let mut direction_modifier = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    ).unwrap();

    set_fullscreen(true);

    loop {
        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );
        gl_use_default_material();

        match state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    reload_val = 100f32;
                    score = 0;
                    state = GameState::Started;
                }
                let text = "Press space";
                let dim = measure_text(text, None, 25, 1.0);
                draw_text(text,
                    screen_width() / 2.0 - dim.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE);
                draw_help();
            }
            GameState::Started => {
                let delta_time = get_frame_time();
                gen_time_cnt += delta_time as f64;
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Paused;
                }
                if is_key_pressed(KeyCode::Space) {
                    if reload_val > 5.0 {
                        reload_val -= 5f32;
                        bullets.push(Shape {
                            size: 32.0,
                            speed: 2.0 * circle.speed,
                            x: circle.x,
                            y: circle.y - 24.0,
                            collided: false,
                            color: RED,
                        })
                    };
                }
                ship_sprite.set_animation(0);
                if is_key_down(KeyCode::Right) {
                    circle.x += circle.speed * delta_time;
                    direction_modifier += 0.05 * delta_time;
                    ship_sprite.set_animation(2);
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= circle.speed * delta_time;
                    direction_modifier -= 0.05 * delta_time;
                    ship_sprite.set_animation(1);
                }
                if is_key_down(KeyCode::Down) {
                    circle.y += circle.speed * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    circle.y -= circle.speed * delta_time;
                }

                circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
                circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

                if gen_time_cnt >= GEN_TIME_CNT_MAX {
                    gen_time_cnt -= GEN_TIME_CNT_MAX;
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

                ship_sprite.update();
                bullet_sprite.update();

                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                squares.retain(|square| square.y < screen_height() + square.size);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size);

                reload(&mut reload_val, delta_time);

                if squares.iter().any(|square| { circle.collides_with(&square) }) {
                    state = GameState::GameOver;
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                }
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if square.collides_with(&bullet) {
                            square.collided = true;
                            bullet.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 4,
                                    texture: Some(explosion_texture.clone()),
                                    ..particle_explosion()
                                }),
                                vec2(square.x, square.y),
                            ));
                        }
                    }
                }

                let ship_frame = ship_sprite.frame();
                draw_texture_ex(
                    &ship_texture,
                    circle.x - ship_frame.dest_size.x,
                    circle.y - ship_frame.dest_size.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(ship_frame.dest_size * 2.0),
                        source: Some(ship_frame.source_rect),
                        ..Default::default()
                    }
                );
                for square in &squares {
                    draw_rectangle(
                        square.x - square.size / 2.0,
                        square.y - square.size / 2.0,
                        square.size,
                        square.size,
                        square.color);
                }
                let bullet_frame = bullet_sprite.frame();
                for bullet in &bullets {
                   draw_texture_ex(
                       &bullet_texture,
                       bullet.x - bullet.size / 2.0,
                       bullet.y - bullet.size / 2.0,
                       WHITE,
                       DrawTextureParams {
                           dest_size: Some(vec2(bullet.size, bullet.size)),
                           source: Some(bullet_frame.source_rect),
                           ..Default::default()
                       }
                   );
                }
                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }

                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0, 35.0, 25.0,
                    WHITE);
                let text_dim = measure_text(format!("Highscore: {}", high_score).as_str(), None, 25, 1.0);
                draw_text(
                    format!("Highscore: {}", high_score).as_str(),
                    screen_width() - text_dim.width - 10.0, 35.0, 25.0,
                    WHITE);

            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Started;
                }
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                let text = "Paused";
                let text_dim = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dim.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED);
                draw_help();
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::MainMenu;
                }
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                let text = "GAME OVER";
                let text_dim = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dim.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED);
                if score == high_score {
                    let text_dim = measure_text("Congrats!!! You reached the high_score!",
                        None, 50, 1.0);
                    draw_text(
                        "Congrats!!! You reached the high_score!",
                        screen_width() / 2.0 - text_dim.width / 2.0,
                        screen_height() / 2.0 + 35.0,
                        50.0,
                        RED);
                }
                draw_help();
            }
        }

        next_frame().await;
    }
}
