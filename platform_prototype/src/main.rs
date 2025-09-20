use std::collections::HashMap;
use std::collections::VecDeque;
use raylib::prelude::*;
use raylib::core::audio::{ Music, RaylibAudio };

use platform_prototype::*;

fn load_background(rl: &mut RaylibHandle, t: &RaylibThread) -> Vec<Texture2D> {
    let mut v = Vec::new();
    let whole_set = Image::load_image("assets/CaveBackground/cave_background.png").unwrap();

    for idx in 0..4 {
        let i = whole_set.from_image(rrect(0 + 320 * idx, 0, 320, 180));
        let t = rl.load_texture_from_image(t, &i).unwrap();
        v.push(t);
    }

    v
}

//fn load_characters(rl: &mut RaylibHandle, t: &RaylibThread) -> HashMap<CharacterId, Character> {
//    let mut retval = HashMap::new();
//
//    let archer= load_character(rl, t, "Archer");
//    let armored_axeman= load_character(rl, t, "Armored Axeman");
//    let armored_orc= load_character(rl, t, "Armored Orc");
//    let armored_skeleton= load_character(rl, t, "Armored Skeleton");
//    let elite_orc= load_character(rl, t, "Elite Orc");
//
//    let greatsword_skeleton = load_character(rl, t,"Greatsword Skeleton");
//    let knight = load_character(rl, t,"Knight");
//    //let knight_templar = load_character(rl, t,"Knight Templar");
//    //let lancer = load_character(rl, t, "Lancer");
//    let orc = load_character(rl, t,"Orc");
//    let orc_rider = load_character(rl, t,"Orc rider");
//    //let priest = load_character(rl, t,"Priest");
//    let skeleton = load_character(rl, t,"Skeleton");
//    //let skeleton_archer = load_character(rl, t,"Skeleton Archer");
//    let slime = load_character(rl, t,"Slime");
//    let soldier = load_character(rl, t,"Soldier");
//    let swordsman = load_character(rl, t,"Swordsman");
//
//    let werebear = load_character(rl, t,"Werebear");
//    let werewolf = load_character(rl, t,"Werewolf");
//    let wizard = load_character(rl, t,"Wizard");
//
//    retval.insert(CharacterId::Archer, archer);
//    retval.insert(CharacterId::ArmoredAxeman, armored_axeman);
//    retval.insert(CharacterId::ArmoredOrc, armored_orc);
//    retval.insert(CharacterId::ArmoredSkeleton, armored_skeleton);
//    retval.insert(CharacterId::EliteOrc, elite_orc);
//    retval.insert(CharacterId::GreatswordSkeleton, greatsword_skeleton);
//    retval.insert(CharacterId::Knight, knight);
//    //retval.insert(CharacterId::KnightTemplar, knight_templar);
//    //retval.insert(CharacterId::Lancer, lancer);
//    retval.insert(CharacterId::Orc, orc);
//    retval.insert(CharacterId::OrcRider, orc_rider);
//    //retval.insert(CharacterId::Priest, priest);
//    retval.insert(CharacterId::Skeleton, skeleton);
//    //retval.insert(CharacterId::SkeletonArcher, skeleton_archer);
//    retval.insert(CharacterId::Slime, slime);
//    retval.insert(CharacterId::Soldier, soldier);
//    retval.insert(CharacterId::Swordsman, swordsman);
//
//    retval.insert(CharacterId::Werebear, werebear);
//    retval.insert(CharacterId::Werewolf, werewolf);
//    retval.insert(CharacterId::Wizard, wizard);
//
//    retval
//}

fn load_character(rl: &mut RaylibHandle, t: &RaylibThread, name: &str) -> CharacterTx {
    let characters_root = "assets/Characters/Characters";
    let file_idle = format!("{}/{}/{}/{}-Idle.png", characters_root, name, name, name);
    let file_walk = format!("{}/{}/{}/{}-Walk.png", characters_root, name, name, name);
    let file_attack1 = format!("{}/{}/{}/{}-Attack01.png", characters_root, name, name, name);
    let file_attack2 = format!("{}/{}/{}/{}-Attack02.png", characters_root, name, name, name);
    let file_death = format!("{}/{}/{}/{}-Death.png", characters_root, name, name, name);
    let file_hurt = format!("{}/{}/{}/{}-Hurt.png", characters_root, name, name, name);

    let idle = load_character_mode(rl, t, &file_idle);
    let walk = load_character_mode(rl, t, &file_walk);
    let attack1 = load_character_mode(rl, t, &file_attack1);
    let attack2 = load_character_mode(rl, t, &file_attack2);
    let death = load_character_mode(rl, t, &file_death);
    let hurt = load_character_mode(rl, t, &file_hurt);

    CharacterTx {
        idle,
        walk,
        attack1,
        attack2,
        death,
        hurt,
    }
}

fn load_character_mode(rl: &mut RaylibHandle, t: &RaylibThread, file: &str) -> Vec<Texture2D> {
    let image = Image::load_image(&file) .unwrap();

    let mut mode = Vec::new();
    let frames = image.width / 32;
    for idx in 0..frames {
        let i = image.from_image(rrect(32 + idx as u32 * 100, 32, 32, 32));
        let t = rl.load_texture_from_image(t, &i).unwrap();
        mode.push(t);
    }

    mode
}

fn normalize_pos(x_pos: &mut i32, y_pos: &mut i32, shift: &mut i32) {
    if x_pos < &mut 10 {
        *shift -= 1;
        *x_pos = 10;
    }

    if x_pos > &mut 700 {
        *shift += 1;
        *x_pos = 700;
    }
}

fn get_input(rl: &RaylibHandle, i: &mut Input) {
    i.up = false;
    i.down = false;
    if rl.is_key_down(KeyboardKey::KEY_W) {
        i.up = true;
    } else if rl.is_key_down(KeyboardKey::KEY_S) {
        i.down = true;
    }

    i.left = false;
    i.right = false;
    if rl.is_key_down(KeyboardKey::KEY_A) {
        i.left = true
    } else if rl.is_key_down(KeyboardKey::KEY_D) {
        i.right = true;
    }

    i.attack1 = false;
    i.attack2 = false;
    if rl.is_key_pressed(KeyboardKey::KEY_Q) {
        i.attack1 = true;
    } else if rl.is_key_pressed(KeyboardKey::KEY_E) {
        i.attack2 = true;
    }

    i.mouse_lb_pressed = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    i.mouse_delta = rl.get_mouse_delta();

    // Gamepad
//    if rl.is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) {
//        y_pos -= speed;
//    } else if rl.is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
//        y_pos += speed;
//    } else if rl.is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
//        x_pos -= speed;
//    } else if rl.is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
//        x_pos += speed;
//    }
}

struct Projectile {
    pos: (f32, f32),
    ttl: u32,
    dir: Direction,
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let mut player_animation_queue: VecDeque<CharacterMode> = VecDeque::new();
    let mut animation_idx: usize = 0;
    let mut animation_idx_last: usize = 0;
    let mut projectiles: VecDeque<Projectile> = VecDeque::new();
    projectiles.push_back(Projectile { pos: (32 as f32 , 30 as f32), ttl: 150, dir: Direction::Right});

    let mut input = Input {
        left: false,
        right: false,
        up: false,
        down: false,
        attack1: false,
        attack2: false,
        mouse_lb_pressed: false,
        mouse_delta: Vector2::zero(),
    };

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Archers")
        .build();

    //let rl_audio = RaylibAudio::init_audio_device().unwrap();
    //let mut music = rl_audio.new_music("assets/Music/dunes-of-silence.ogg").unwrap();
    //let mut music = rl_audio.new_music("assets/Music/2. Scorchlight Mirage.ogg").unwrap();
    //let mut sound1 = rl_audio.new_sound("assets/SFX/Attacks/bow/bow_attack_1.ogg").unwrap();
    //music.play_stream();

    let origin = Vector2 { x: 0 as f32, y: 0 as f32 };

    let backgrounds = load_background(&mut rl, &thread);
    let mut character_idx: usize = 0;
    let character_ids = vec![
        CharacterId::Archer,
        CharacterId::ArmoredAxeman,
        CharacterId::ArmoredOrc,
        CharacterId::ArmoredSkeleton,
        CharacterId::EliteOrc,
        CharacterId::GreatswordSkeleton,
        CharacterId::Knight,
        CharacterId::Orc,
        CharacterId::OrcRider,
        CharacterId::Skeleton,
        CharacterId::Slime,
        CharacterId::Soldier,
        CharacterId::Swordsman,
        CharacterId::Werebear,
        CharacterId::Werewolf,
        CharacterId::Wizard,
    ];
    //let characters = load_characters(&mut rl, &thread);
    let archer_tx= load_character(&mut rl, &thread, "Archer");
    let arrow_img = Image::load_image("assets/Characters/Arrow/Arrow01(32x32).png") .unwrap();
    //let arrow_img = Image::load_image("assets/Characters/Arrow/Arrow04(32x32).png") .unwrap();
    let arrow_tx = rl.load_texture_from_image(&thread, &arrow_img).unwrap();

    let mut tick = 0;
    let (mut idle_idx, mut walk_idx)= (0, 0);
    let (mut attack1_idx, mut attack2_idx) = (0, 0);
    let (mut hurt_idx, mut death_idx) = (0, 0);
    let (mut x_pos, mut y_pos) = (400, 300);
    let mut background_shift = 0;
    let mut dir = Direction::Right;
    let mut player_mode;
    rl.set_target_fps(60);

    let speed = 4;

    {
        let mut d = rl.begin_drawing(&thread);
    }
    for id in 0..4 {
        println!("is gamepad[{}] avail: {}", id, rl.is_gamepad_available(id));
    }

    let mut player = Character::new(CharacterId::Archer, (200f32, 200f32));

    loop {
        //music.update_stream();
        //let current_character = characters.get(&character_ids[character_idx]).unwrap();

        if rl.window_should_close() {
            break;
        }

        get_input(&rl, &mut input);
        player.kick(&input);

        if input.left == false && input.right == false && input.up == false && input.down == false{
            player_mode = CharacterMode::Idle;
        } else {
            player_mode = CharacterMode::Walk;
        }

        if input.left {
            dir = Direction::Left;
            x_pos -= speed;
            if x_pos < 0 {
                x_pos = 800;
            }
        }
        if input.right {
            dir = Direction::Right;
            x_pos += speed;
            if x_pos > 800 {
                x_pos = 0;
            }
        }
        if input.up {
            y_pos -= speed;
            if y_pos < 0 {
                y_pos = 600;
            }
        }
        if input.down {
            y_pos += speed;
            if y_pos > 600 {
                y_pos = 0;
            }
        }

        //normalize_pos(&mut x_pos, &mut y_pos, &mut background_shift);

        if input.attack1 {
            player_animation_queue.push_back(CharacterMode::Attack1);
            // TODO: get this from already read assets
            animation_idx = 0;
            animation_idx_last = 8;
        }

        if input.attack2 {
            player_animation_queue.push_back(CharacterMode::Attack2);
            // TODO: get this from already read assets
            animation_idx = 0;
            animation_idx_last = 11;
        }

        if tick % 8 == 0 {
            idle_idx += 1;
            if idle_idx > 5 {
                idle_idx = 0;
            }

            walk_idx += 1;
            if walk_idx > 7 {
                walk_idx = 0;
            }

            animation_idx += 1;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        for idx in 0..4 {
            d.draw_texture_pro(&backgrounds[idx],
                rrect(background_shift * (1 + idx) as i32, 0, 320, 180),
                rrect(0, 0, 320 * 4, 180 * 4),
                origin,
                0 as f32,
                Color::WHITE);
        }

        // Draw player
        if player_animation_queue.is_empty() {
            if player_mode == CharacterMode::Idle {
                player.draw_character(&mut d, &archer_tx.idle[idle_idx], &input); // TODO:
                                                                                     // textures
                                                                                     // passed in?
            } else {
                player.draw_character(&mut d, &archer_tx.walk[walk_idx], &input); // TODO:
            }
        } else {
            let anim = match player_animation_queue[0] {
                CharacterMode::Attack1 => &archer_tx.attack1,
                CharacterMode::Attack2 => &archer_tx.attack2,
                _ => &archer_tx.death, // TODO: temporary
            };
            player.draw_character(&mut d, &anim[animation_idx], &input); // TODO:

            if animation_idx >= animation_idx_last {
                let (arrowdir, start_x_pos) = match dir {
                    Direction::Left => (Direction::Left, x_pos + -32),
                    Direction::Right => (Direction::Right, x_pos + 32),
                };
                animation_idx = 0;
                player_animation_queue.pop_front().unwrap();
                    projectiles.push_back(Projectile { pos: (start_x_pos as f32 , y_pos as f32), ttl: 150, dir: arrowdir});
            }

//            if animation_idx == 4 {
//                if !sound1.is_playing() {
//                    sound1.play();
//                }
//            }
        }

        // Draw projectiles
        d.draw_text(&format!("num of projectiles: {}", projectiles.len()), 10, 10, 10, Color::RED);
        if !projectiles.is_empty() {
            for p in projectiles.iter_mut() {
                let speed = 5f32;
                let scale = 2f32;
                let origin = Vector2 { x: scale * ( arrow_tx.width / 2) as f32, y: scale * (arrow_tx.height / 2) as f32 };
                    let arrow_width = match p.dir {
                        Direction::Left => -arrow_tx.width,
                        Direction::Right => arrow_tx.width,
                    };
                d.draw_texture_pro(
                    &arrow_tx,
                    rrect(0, 1, arrow_width, arrow_tx.height),
                    rrect(p.pos.0, p.pos.1, scale as i32 * arrow_tx.width, scale as i32 * arrow_tx.height),
                    origin,
                    0 as f32,
                    Color::WHITE);

           //     d.draw_circle(
           //         p.pos.0 as i32,
           //         p.pos.1 as i32,
           //         5f32, Color::BLUE);

                if p.ttl > 0 {
                    p.ttl -= 1;
                    match p.dir {
                        Direction::Left => p.pos.0 -= speed,
                        Direction::Right => p.pos.0 += speed,
                    }
                }

                if p.pos.0 > 800f32 {
                    p.pos.0 = 0f32;
                } else if p.pos.0 < 0f32 {
                    p.pos.0 = 800f32;
                }
            }
            projectiles.retain(|el| el.ttl > 0);
        }

        tick += 1;

        d.gui_label(
            rrect(50, 50, 100, 30),
            "Entities - label"
        );

        if d.gui_button( rrect(50, 80, 100, 30), "Step") {
            println!("Button pressed!");
        }

                
        
    }

}
