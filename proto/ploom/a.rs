use nannou::prelude::*;
use rand::distributions::{Distribution, Normal};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    circles: Vec<Circle>,
}

struct Circle {
    x: Vec2,
    r: f32,
}

impl Circle {
    fn new(x: Vec2, r: f32) -> Circle {
        Circle { x, r }
    }

    fn random_new() -> Circle {
        let s: f32 = 200.0;
        let r_max: f32 = 10.0;
        let normal = Normal::new(0.0, s as f64);
        let x: Vec2 = vec2(
            normal.sample(&mut rand::thread_rng()) as f32,
            normal.sample(&mut rand::thread_rng()) as f32,
        );
        Circle::new(x, random_range(0.0, r_max))
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();

    let num_circles: usize = 100;
    let circles: Vec<Circle> = spawn_random_circles(num_circles);
    Model { circles }
}

fn spawn_random_circles(num_circles: usize) -> Vec<Circle> {
    (0..num_circles).map(|_| Circle::random_new()).collect()
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    return;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
        draw.background().color(WHITE);
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(0.0, 0.0, 0.0, 0.07);
    }

    model.circles.iter().for_each(|circle| {
        draw.ellipse()
            .xy(circle.x)
            .radius(circle.r)
            .color(LIGHTSTEELBLUE);
    });

    draw.to_frame(app, &frame).unwrap();
}

fn key_released(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}
