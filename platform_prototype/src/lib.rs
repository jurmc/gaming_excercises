use raylib::prelude::*;

fn normalize_angle(a: &mut f32) {
    if *a > 360f32 {
        *a -= 360.0;
    } else if *a < 0.0 {
        *a += 360.0;
    }
}

pub enum Direction {
    Left,
    Right,
}

pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub attack1: bool,
    pub attack2: bool,
    pub mouse_lb_pressed: bool,
    pub mouse_delta: Vector2,
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum CharacterMode {
    Idle,
    Walk,
    Jump,
    Fall,
    Attack1,
    Attack2,
    Hurt,
    Death,
}

#[derive(PartialEq, Hash, Eq)]
pub enum CharacterId {
    Archer,
    ArmoredAxeman,
    ArmoredOrc,
    ArmoredSkeleton,
    EliteOrc,
    GreatswordSkeleton,
    Knight,
    //KnightTemplar,
    //Lancer,
    Orc,
    OrcRider,
    Priest,
    Skeleton,
    SkeletonArcher,
    Slime,
    Soldier,
    Swordsman,
    Werebear,
    Werewolf,
    Wizard,
}

pub struct CharacterTx {
    pub idle: Vec<Texture2D>,
    pub walk: Vec<Texture2D>,
    pub attack1: Vec<Texture2D>,
    pub attack2: Vec<Texture2D>,
    pub hurt: Vec<Texture2D>,
    pub death: Vec<Texture2D>,
}

pub struct Character {
    pub id: CharacterId,
    pub pos: (f32, f32), // TODO: convert to Vector2
    pub vel: Vector2,
    pub acc: Vector2,
    dir: Direction,
    mode: CharacterMode,
    angle: f32,
}

impl Character{
    pub fn new(id: CharacterId, pos: (f32, f32)) -> Character {
        Character {
            id,
            pos,
            vel: rvec2(0.0, 0.0),
            acc: rvec2(0.0, 0.2),
            dir: Direction::Right,
            mode: CharacterMode::Idle,
            angle: 0.0,
        }
    }

    pub fn kick(&mut self, i: &Input) {
        let speed = 4f32;

        if self.pos.1 < 500f32 {
            self.vel.y += self.acc.y;
        } else if i.up == true {
            self.vel.y = -10.0;
        } else {
            self.vel.y = 0.0;
        }

        self.pos.1 += self.vel.y;

        if i.left {
            self.dir = Direction::Left;
            self.pos.0 -= speed;
            if self.pos.0 < 0f32 {
                self.pos.0 = 800f32;
            }
        }
        if i.right {
            self.dir = Direction::Right;
            self.pos.0 += speed;
            if self.pos.0 > 800f32 {
                self.pos.0 = 0f32;
            }
        }

        if i.mouse_lb_pressed {
            self.angle += i.mouse_delta.x;
            normalize_angle(&mut self.angle);
            if self.angle < 180.0 && self.angle > 0.0 {
                if self.angle < 90.0 {
                    self.angle += 0.0;
                    self.dir = Direction::Right
                } else {
                    self.dir = Direction::Left
                }
            } else if self.angle > 270.0 {
                self.dir = Direction::Right;
            } else {
                self.dir = Direction::Left;
            }
            println!("angle: {}", self.angle);
        } else {
            self.angle = 0.0;
        }
         
    }

    pub fn draw_character(&self, d: &mut RaylibDrawHandle<'_>, t: &Texture2D, i: &Input) {
        let scale = 3f32;
        let mut character_src_width = match self.dir { // TODO: can be made member var, initialized in
                                                   // ::new()
            Direction::Left => -t.width,
            Direction::Right => t.width,
        };

        let origin = Vector2 { x: scale * ( t.width / 2) as f32, y: scale * (t.height / 2) as f32 };

        let angle = match self.dir {
            Direction::Left => 360.0 - self.angle,
            Direction::Right => self.angle,
        };
        let (x, y) = (self.pos.0 as i32, self.pos.1 as i32);
        d.draw_texture_pro(t,
            rrect(0, 0, character_src_width, t.height),
            rrect(x, y, scale as i32 * t.width, scale as i32 * t.height),
            origin,
            angle,
            Color::WHITE);

        let c = Color::YELLOWGREEN;
        d.draw_line(x - 25, y, x + 25, y, c);
        d.draw_line(x, y - 25, x, y + 25, c);
        d.draw_circle_lines(x, y, scale * (t.width / 4) as f32, c);

        d.draw_text(&format!("x: {}, y: {}", x, y), x+32, y+10, 10, c);
        d.draw_text(&format!("mode: {:?}", self.mode), x+32, y+22, 10, c);

    }
}

