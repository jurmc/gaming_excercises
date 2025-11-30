use macroquad::prelude::*;
use gilrs::{Gilrs, Button, Event};

const SQUARE_SIZE: u32 = 25;

fn draw_square_for_point(p: Vec2, c: Color) {
    let sq_p = Vec2::new(
        (SQUARE_SIZE * ((p.x as u32) / SQUARE_SIZE)) as f32,
        (SQUARE_SIZE * ((p.y as u32) / SQUARE_SIZE)) as f32);

    draw_rectangle(sq_p.x, sq_p.y, SQUARE_SIZE as f32, SQUARE_SIZE as f32, c);
}

struct Input {
    dir: Vec2,
    rot: f32,
    l: f32,
}

fn get_input() -> Input {
    if is_key_down(KeyCode::Q) {
        std::process::exit(0); 
    }

    let mut v = 2.0;

    let mut dir = Vec2::new(0., 0.);
    if is_key_down(KeyCode::W) {
        dir.y = -1.0;
    }
    if is_key_down(KeyCode::A) {
        dir.x = -1.0;
    }
    if is_key_down(KeyCode::S) {
        dir.y = 1.0;
    }
    if is_key_down(KeyCode::D) {
        dir.x = 1.0;
    }
    dir *= v;

    let mut rot = 0.0f32;
    if is_key_down(KeyCode::Left) {
        rot = -1.0;
    }
    if is_key_down(KeyCode::Right) {
        rot = 1.0;
    }

    let mut l = 0.;
    if is_key_down(KeyCode::Up) {
        l = 1.0;
    }
    if is_key_down(KeyCode::Down) {
        l = -1.0;
    }
    let rot = rot.to_radians();

    Input { dir, rot, l }
}

struct Player {
    pos: Vec2,
    rot: f32,
    l: f32,
}

impl Player {
    fn get_tip(&self) -> Vec2 {
        let a = Affine2:: from_angle(self.rot);
        let tip = a.transform_point2(Vec2::new(self.l, 0.));
        self.pos + tip
    }

    fn draw(&self) {
        let tip = self.get_tip();
        draw_line(self.pos.x, self.pos.y, tip.x, tip.y, 2.0, GREEN);
        draw_circle(self.pos.x, self.pos.y, 5., GREEN);
        draw_circle(tip.x, tip.y, 5., ORANGE);

    }

}

#[macroquad::main("MyGame")]
async fn main() {
    let mut player = Player {
        pos: Vec2::new(0., 0.),
        rot: 45.0f32.to_radians(),
        l: 150.,
    };

    loop {
        let i = get_input();
        player.pos += i.dir;
        player.rot += i.rot;
        //l += i.l;

        //let a = Affine2:: from_angle(rotation);
        //let cannon = get_cannon(player, l, a);

        clear_background(DARKGRAY);

        for x in 0..30 {
            for y in 0..20 {
                draw_rectangle_lines(
                    (x * SQUARE_SIZE) as f32,
                    (y * SQUARE_SIZE) as f32,
                    SQUARE_SIZE as f32,
                    SQUARE_SIZE as f32,
                    1.0, GRAY);
            }
        }

        draw_square_for_point(player.pos, RED);
        draw_square_for_point(player.get_tip(), PURPLE);
        player.draw();

        next_frame().await
    }
}
