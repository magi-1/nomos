use nannou::prelude::*;
use rand::distributions::{Distribution, Normal};

const WINDOW_SIZE: u32 = 1000;
const NUM_PLOOMS: usize = 200;
const NUM_CIRCLES: usize = 5;
const MAX_RADIUS: f32 = 3.0;
const SCALE: f32 = 100.0;

const DECAY: f32 = 0.99; // variance decay
const THRESHOLD: f32 = 0.5; // min variance

const PLOOM: f32 = SCALE / 5.0; // or 4.0, or 10
const DAMPENING: f32 = 0.01;
const ALPHA: f32 = 0.02;

fn main() {
    nannou::app(model).update(update).run();
}

fn rand_normal_vec2(mu: f32, var: f32) -> Vec2 {
    let normal = Normal::new(mu as f64, var as f64);
    vec2(
        normal.sample(&mut rand::thread_rng()) as f32,
        normal.sample(&mut rand::thread_rng()) as f32,
    )
}

fn rand_uniform_vec2() -> Vec2 {
    let bound = WINDOW_SIZE as f32 / 2.0;
    vec2(random_range(-bound, bound), random_range(-bound, bound))
}

struct Circle {
    x: Vec2,
    r: f32,
}

struct Ploom {
    circles: Vec<Circle>,
    focus: Vec2,
    sigma: f32,
}

impl Ploom {
    fn new() -> Ploom {
        Ploom {
            circles: Circle::spawn_random_circles(NUM_CIRCLES),
            focus: rand_uniform_vec2(),
            sigma: PLOOM,
        }
    }

    fn spawn_random_plooms(n: usize) -> Vec<Ploom> {
        (0..n).map(|_| Ploom::new()).collect()
    }

    fn update(&mut self) -> bool {
        if self.sigma > THRESHOLD {
            self.sigma *= DECAY;
            false
        } else {
            self.focus = rand_uniform_vec2();
            self.sigma = PLOOM;
            true
        }
    }
}

impl Circle {
    fn new() -> Circle {
        Circle {
            x: rand_normal_vec2(0.0, PLOOM),
            r: random_range(1.0, MAX_RADIUS),
        }
    }

    fn spawn_random_circles(n: usize) -> Vec<Circle> {
        (0..n).map(|_| Circle::new()).collect()
    }

    fn update(&mut self, focus: Vec2, sigma: f32) {
        let mut dx: Vec2 = rand_normal_vec2(0.0, sigma);
        dx[0] /= self.r;
        dx[1] /= self.r;
        self.x += dx - (self.x - focus) * DAMPENING;
    }
}

struct Model {
    plooms: Vec<Ploom>,
    event_bool: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();

    Model {
        plooms: Ploom::spawn_random_plooms(NUM_PLOOMS),
        event_bool: true,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..NUM_PLOOMS {
        model.event_bool = model.plooms[i].update();
        let focus = model.plooms[i].focus;
        let sigma = model.plooms[i].sigma;
        for j in 0..NUM_CIRCLES {
            model.plooms[i].circles[j].update(focus, sigma)
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.event_bool {
        draw.background().color(WHITE);
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(0.18431373, 0.19215686, 0.29019608, ALPHA);
    }

    let colors = vec![LIGHTSTEELBLUE, VIOLET];
    for ploom in &model.plooms {
        for i in 0..ploom.circles.len() {
            draw.ellipse()
                .xy(ploom.circles[i].x)
                .radius(ploom.circles[i].r)
                .color(colors[i % 2]);
        }
    }
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
