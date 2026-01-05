use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const ROWS: u32 = 20;
const COLS: u32 = 20;
const SIZE: u32 = 30;
const MARGIN: u32 = 10;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;
const COLORS: [Rgb<u8>; 10] = [
    PLUM, TEAL, VIOLET, CORAL, GOLD, FORESTGREEN, SLATEBLUE, SALMON, TURQUOISE, AQUAMARINE
];

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    _window: window::Id,
    random_seed: u64,
    disp_adj: f32,
    rot_adj: f32,
    gravel: Vec<Stone>,
    background_color: Rgb<u8>,
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        Stone {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
        }
    }
}

fn model(app: &App) -> Model {
    // let _window = app.new_window().view(view).build().unwrap();
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let random_seed = random_range(0, 1000000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;
    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32);
            gravel.push(stone);
        }
    }

    Model {
        _window,
        random_seed,
        disp_adj,
        rot_adj,
        gravel,
        background_color: COLORS
                [random_range(0, COLORS.len() as i32) as usize],
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let file_path = app.project_path().unwrap().join("output").join(format!(
                "{}-{}.png",
                app.exe_name().unwrap(),
                model.random_seed
            ));
            app.main_window().capture_frame(file_path.clone());
            println!("saved frame");
            println!("{:?}", file_path.display());
        }
        Key::C => {
            model.background_color = COLORS
                [random_range(0, COLORS.len() as i32) as usize];
            println!("changed color to {:?}", model.background_color);
        }
        Key::R => model.random_seed = random_range(0, 1000000),
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for stone in &mut model.gravel {
        let factor = stone.y / ROWS as f32;
        let disp_factor = factor * model.disp_adj;
        let rot_factor = factor * model.rot_adj;
        stone.x_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.y_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.rotation = rot_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        // cdraw
        //     .tri()
        //     .no_fill()
        //     .stroke(BLACK)
        //     .stroke_weight(LINE_WIDTH)
        //     .points(
        //         pt2(0.0, 0.0),
        //         pt2(1.0, 0.0),
        //         pt2(0.5, (2.0f32).sqrt() / 2.0),
        //     )
        //     .x_y(0.0, 0.0)
        //     .rotate(0.0);
        cdraw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation);
    }
    draw.background().color(model.background_color);
    draw.to_frame(app, &frame).unwrap();
}
