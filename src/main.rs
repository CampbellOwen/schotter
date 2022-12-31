use nannou::prelude::*;

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let draw_grid_space = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    for y in 0..ROWS {
        for x in 0..COLS {
            let chaos_factor = (y as f32 / ROWS as f32) * 0.75;
            let x_offset = chaos_factor * random_range(-0.5, 0.5);
            let y_offset = chaos_factor * random_range(-0.5, 0.5);
            let rotation = chaos_factor * random_range(-PI / 4.0, PI / 4.0);

            let draw_square_space = draw_grid_space.x_y(x as f32, y as f32);
            draw_square_space
                .rect()
                .no_fill()
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
