use macroquad::prelude::*;

const SQUARE_SIZE: u32 = 41;

fn draw_vec(p1: Vec2, p2: Vec2, t: Affine2) {
    let a1 = t.transform_point2(Vec2::new(p2.x - 10., p2.y + 10.));
    let a2 = t.transform_point2(Vec2::new(p2.x - 10., p2.y - 10.));

    let p1 = t.transform_point2(p1); 
    let p2 = t.transform_point2(p2);
    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, GREEN);
    draw_line(p2.x, p2.y, a1.x, a1.y, 2.0, GREEN);
    draw_line(p2.x, p2.y, a2.x, a2.y, 2.0, GREEN);

}

fn draw_square_for_point(p: Vec2, t: Affine2, c: Color) {
    let p = t.transform_point2(Vec2::new(p.x, p.y));

    let sq_p = Vec2::new(
        (SQUARE_SIZE * ((p.x as u32) / SQUARE_SIZE)) as f32,
        (SQUARE_SIZE * ((p.y as u32) / SQUARE_SIZE)) as f32);

    draw_rectangle(sq_p.x, sq_p.y, SQUARE_SIZE as f32, SQUARE_SIZE as f32, c);
}

#[macroquad::main("MyGame")]
async fn main() {
    let p1 = Vec2::new(0., 0.);
    let p2 = Vec2::new(100., 0.);

    let mut rx: f32 = 250.;
    let mut ry: f32 = 180.;
    let mut cx: f32 = 400.;
    let mut cy: f32 = 300.;

    loop {
        let t = get_time() as f32;
        let bx = cx + rx*(t/3.).cos();
        let by = cy + ry*(t/2.).sin();
        let translation =Vec2::new(bx, by);
        let a1 = Affine2:: from_angle_translation(
            t as f32,
            translation);
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

        draw_square_for_point(p1, a1, RED);
        draw_square_for_point(p2, a1, PURPLE);
        draw_vec(p1, p2, a1);

        next_frame().await
    }
}
