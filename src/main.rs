use nannou::{
    prelude::*,
    rand::{rngs::StdRng, Rng, SeedableRng},
    winit::event::WindowEvent,
};

use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run()
}

struct Model {
    pub main_window: WindowId,
    pub ui: Egui,
    pub random_seed: u64,
    pub displace_adjust: f32,
    pub rotation_adjust: f32,
    pub gravel: Vec<Stone>,
}

#[derive(Debug, Default, Clone)]
struct Stone {
    pos: (f32, f32),
    offset: (f32, f32),
    rotation: f32,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 130)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    let random_seed = random_range(0, 1000000);
    let displace_adjust = 1.0;
    let rotation_adjust = 1.0;

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            gravel.push(Stone {
                pos: (x as f32, y as f32),
                ..Default::default()
            });
        }
    }

    Model {
        main_window: window,
        ui,
        random_seed,
        displace_adjust,
        rotation_adjust,
        gravel,
    }
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}
fn raw_ui_event(_app: &App, model: &mut Model, event: &WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1_000_000);
        }
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(app.exe_name().unwrap() + ".png")
            }
        }
        Key::Up => {
            model.displace_adjust += 0.1;
        }
        Key::Down => {
            if model.displace_adjust > 0.0 {
                model.displace_adjust -= 0.1;
            }
        }
        Key::Right => {
            model.rotation_adjust += 0.1;
        }
        Key::Left => {
            if model.rotation_adjust > 0.0 {
                model.rotation_adjust -= 0.1;
            }
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for stone in &mut model.gravel {
        let chaos_factor = stone.pos.1 / ROWS as f32;

        let displacement_factor = chaos_factor * model.displace_adjust;
        let rotation_factor = chaos_factor * model.rotation_adjust;
        stone.offset.0 = displacement_factor * rng.gen_range(-0.5..0.5);
        stone.offset.1 = displacement_factor * rng.gen_range(-0.5..0.5);
        stone.rotation = rotation_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
    }
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.displace_adjust, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rotation_adjust, 0.0..=5.0).text("Displacement"));
            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    model.random_seed = random_range(0, 1_000_000);
                }

                ui.add_space(20.0);
                ui.add(egui::DragValue::new(&mut model.random_seed));
                ui.label("Seed");
            });
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let draw_grid_space = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    for stone in &model.gravel {
        let draw_square_space = draw_grid_space.x_y(stone.pos.0, stone.pos.1);
        draw_square_space
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.offset.0, stone.offset.1)
            .rotate(stone.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}
