extern crate raylib;
use raylib::prelude::*;
use std::collections::HashMap;
use std::cmp::Eq;

const MAX_BLOCS_X: i32 = 10;
const MAX_BLOCS_Y: i32 = 8;
const BLOCK_SIZE: i32 = 64;
const WIDTH: i32 = MAX_BLOCS_X * BLOCK_SIZE;
const HEIGHT: i32 = MAX_BLOCS_Y * BLOCK_SIZE;

#[derive(Debug, Eq, PartialEq, Hash)]
enum BlockType {
    Dirt,
    Grass,
}

#[derive(Debug)]
struct Block {
    x: i32,
    y: i32,
    t: BlockType,
}

fn load_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<BlockType, Texture2D> {
    let mut textures = HashMap::new();
    let t = rl
        .load_texture(&thread, "/home/jurmc/game_assets/SBS/Textures/Dirt/cubeDirt_1.png")
        .expect("could not load texture: cubeDirt_1.png");
    textures.insert(BlockType::Dirt, t);
    let t = rl
        .load_texture(&thread, "/home/jurmc/game_assets/SBS/Textures/Grass/cubeGreen_1.png")
        .expect("could not load texture: cubeGreen_1.png");
    textures.insert(BlockType::Grass, t);

    textures
}

fn load_character(rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    // Character
    let knight_frames =  Image::
        load_image("/home/jurmc/game_assets/Tiny RPG Character Asset Pack v1.03 -Full 20 Characters/Characters(100x100)/Knight/Knight/Knight-Idle.png")
        .expect("could not load image: Knight-Idle.png");
    let knigth_frame = Image::
        from_image(&knight_frames, rrect(38, 32, 32, 32)); 
    let t = rl
        .load_texture_from_image(&thread, &knigth_frame)
        .expect("could not load texture from Knight image");

    t
}


fn load_blocks() -> HashMap<(i32, i32), Block> {
    let columns = [3, 2, 7, 6, 3, 4, 5, 4, 2, 2];
    let mut blocks = HashMap::new();

    for x in 0..columns.len() {
        let column_height = columns[x] as usize;
        for y in 0..column_height {
            let mut bt = BlockType::Dirt;
            if y == column_height - 1 {
                bt = BlockType::Grass;
            };

            let b = Block { x: x as i32, y: y as i32, t: bt };
            blocks.insert((b.x, b.y), b);
        }
    }
    blocks
}

fn draw_block(d: &mut RaylibDrawHandle<'_>, b: &Block, t: &Texture2D) {
    let (x1, y1) = (b.x * BLOCK_SIZE, HEIGHT - b.y * BLOCK_SIZE - BLOCK_SIZE);
    d.draw_texture(&t, x1, y1, Color::WHITE);
    let r = rrect(x1, y1, BLOCK_SIZE, BLOCK_SIZE);
    d.draw_rectangle_lines_ex(r, 2f32, Color::RED);
}

fn draw_pointer(d: &mut RaylibDrawHandle<'_>, pos: Vector2, dir: Vector2) {
    d.draw_line(pos.x as i32, pos.y as i32,
        (pos.x + dir.x) as i32, (pos.y + dir.y) as i32,
        Color::BLUE);
}

fn main() {
    let (w, h) = (WIDTH, HEIGHT);
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("2d geometry")
        .build();
    rl.toggle_fullscreen();
    let rust_orange = Color::new(222, 165, 132, 255);
    let ray_white = Color::new(255, 255, 255, 255);

    let textures = load_textures(&mut rl, &thread);
    let blocks = load_blocks();
    let character = load_character(&mut rl, &thread);

    let mut character_pos = Vector2::new(50f32, 50f32);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let t_sec = rl.get_time();
        rl.draw(&thread, |mut d| {
            d.clear_background(ray_white);

            for (_, b) in &blocks {
                if let Some(t) = textures.get(&b.t) {
                    draw_block(&mut d, b, t);
                } else {
                    eprintln!("Missing testure: {:?}", b.t);
                }
            }

            d.draw_texture_ex(&character, character_pos, 0f32, 2f32, Color::WHITE);

            let direction = Vector2::new(50f32, 20f32);
            draw_pointer(&mut d, character_pos, direction);

            let s = format!("Time elapsed: {:.0}", t_sec);
            d.gui_label(
                rrect(550, 10, 100, 30),
                &s
            );
        });
    }
}
