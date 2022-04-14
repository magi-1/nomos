use nannou::prelude::*;
use rand::distributions::{Normal, Distribution};

const NUM_CIRCLES: usize = 50;
const MAX_RADIUS: f32 = 3.0;
const SCALE: f32 = 100.0;
const PLOOM: f32 = SCALE/5.0; // or 4.0

fn main() {
    nannou::app(model).update(update).run();
}

fn rand_vec2(mu: f32, var: f32) -> Vec2 {
    let normal = Normal::new(mu as f64, var as f64);
    vec2(
        normal.sample(&mut rand::thread_rng()) as f32,
        normal.sample(&mut rand::thread_rng()) as f32
    )
}

struct Model {
    circles: Vec<Circle>,
    focus: Vec2,
}

struct Circle {
    x: Vec2,
    r: f32,
    sigma: f32
}

impl Circle {

    fn random_new() -> Circle {
        Circle {
            x: rand_vec2(0.0, SCALE),
            r: random_range(1.0, MAX_RADIUS),
            sigma: PLOOM
        }
    }

    fn update(&mut self, focus: Vec2) {
        self.sigma = if self.sigma > 0.5 {self.sigma*0.98} else {PLOOM};
        let mut delta: Vec2 = rand_vec2(0.0, self.sigma);
        delta[0] /= self.r;
        delta[1] /= self.r;
        self.x += delta - (self.x-focus)*0.01;
    }
}

fn spawn_random_circles(num_circles: usize) -> Vec<Circle> {
    (0..num_circles).map(|_| {Circle::random_new()}).collect()
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();

    let circles: Vec<Circle> = spawn_random_circles(NUM_CIRCLES);
    let focus: Vec2 = rand_vec2(0.0, SCALE);
    Model {circles, focus}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.circles.len() {
        model.circles[i].update(model.focus);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(0.18431373, 0.19215686, 0.29019608, 0.02);
    }

    let colors = vec![LIGHTSTEELBLUE, VIOLET];
    for i in 0..model.circles.len() {
        draw.ellipse()
        .xy(model.circles[i].x)
        .radius(model.circles[i].r)
        .color(colors[i%2]);
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.focus = rand_vec2(0.0, SCALE)
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}