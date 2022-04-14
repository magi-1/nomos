use nannou::prelude::*;
use rand::distributions::{Normal, Distribution};

const WINDOW_SIZE: u32 = 1000;
const NUM_PLOOMS: usize = 100;
const NUM_CIRCLES: usize = 3;
const MAX_RADIUS: f32 = 3.0;
const SCALE: f32 = 100.0;

const DECAY: f32 = 0.98; // variance decay
const THRESHOLD: f32 = 0.2; // min variance

const PLOOM: f32 = SCALE/5.0; // or 4.0, or 10
const DAMPENING: f32 = 0.02;
const ALPHA: f32 = 0.02;

const MAX_DISTANCE: f32 = 100.0;

fn main() {
    nannou::app(model).update(update).run();
}

fn rand_normal_vec2(mu: f32, var: f32) -> Vec2 {
    let normal = Normal::new(mu as f64, var as f64);
    vec2(
        normal.sample(&mut rand::thread_rng()) as f32,
        normal.sample(&mut rand::thread_rng()) as f32
    )
}

fn rand_uniform_vec2() -> Vec2 {
    let bound = WINDOW_SIZE as f32/2.0;
    vec2(
        random_range(-bound, bound),
        random_range(-bound, bound)
        
    )
}

struct Circle {
    x: Vec2,
    r: f32
}

struct Ploom {
    circles: Vec<Circle>,
    focus: Vec2,
    sigma : f32,
    v: Vec2
}

impl Ploom {

    fn new() -> Ploom {
        Ploom {
            circles: Circle::spawn_random_circles(NUM_CIRCLES),
            focus: rand_uniform_vec2(),
            sigma: PLOOM,
            v: rand_normal_vec2(0.0, 1.0)
        }
    }

    fn spawn_random_plooms(n: usize) -> Vec<Ploom>{
        (0..n).map(|_| {Ploom::new()}).collect()
    }

    fn update(&mut self) -> bool{ 
        if self.sigma > THRESHOLD {
            self.sigma *= DECAY;
            self.focus += self.v; //rand_normal_vec2(0.0, 3.0);
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
        (0..n).map(|_| {Circle::new()}).collect()
    }

    fn update(&mut self, focus: Vec2, sigma: f32) {
        let mut dx: Vec2 = rand_normal_vec2(0.0, sigma);
        dx[0] /= self.r;
        dx[1] /= self.r;
        self.x += dx - (self.x-focus)*DAMPENING;
    }
}

struct Model {
    plooms: Vec<Ploom>,
    event_bool: bool
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
        event_bool: true
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
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(0.0,0.0,0.0, ALPHA);
    }
    
    let time = app.time;
    draw_plumes(&draw, &model, time);
    draw_lines(&draw, &model, time);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_plumes(draw: &Draw, model: &Model, time: f32) {
    let colors = vec![
        //Rgba::new(1.0, 0.96862745, 0.63137255, 0.4), 
        //Rgba::new(0.04, 0.56, 0.99, 0.4),
        Rgba::new(0.63921569, 0.09803922, 0.05882353, 0.4)
        ];

    for ploom in &model.plooms {
        draw.ellipse()
            .xy(ploom.focus)
            .radius(5.0)
            .rgba(1.0,1.0,1.0, time/400.0);

        
        for i in 0..ploom.circles.len() {
            draw.ellipse()
                .xy(ploom.circles[i].x)
                .radius(ploom.circles[i].r)
                .color(colors[i%colors.len()]);
        }
    }
}

fn draw_lines(draw: &Draw, model: &Model, time: f32) {
    for p1 in &model.plooms {
        for p2 in &model.plooms {
            let delta = p1.focus-p2.focus;
            let distance = (delta).dot(delta).sqrt();
            if distance < MAX_DISTANCE {
                draw.line()
                    .start(p1.focus)
                    .end(p2.focus)
                    .weight(2.0)
                    .rgba(1.0,1.0,1.0, time/500.0);
            }
        }
    }
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