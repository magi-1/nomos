use nannou::prelude::*;
use rand::distributions::{Normal, Distribution};

const WINDOW_SIZE: u32 = 1000;
const NUM_CELLS: usize = 2;
const BALL_COUNT: usize = 10;
const BALL_SIZE: f32 = 4.0;

const BALL_V: f32 = 1.0;
const ALPHA: f32 = 0.25;
const DT: f32 = 0.3;


fn rand_normal_vec2(mu: f32, var: f32) -> Vec2 {
    let normal = Normal::new(mu as f64, var as f64);
    vec2(
        normal.sample(&mut rand::thread_rng()) as f32,
        normal.sample(&mut rand::thread_rng()) as f32
    )
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Poly {
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
    p4: Vec2,
    color: Rgba
}

struct Ball {
    loc: Vec2,
    v: Vec2
}

struct Cell {
    poly: Poly,
    balls: Vec<Ball>
}

impl Ball {
    fn new(loc: Vec2) -> Ball {
        Ball {
            loc: loc,
            v: rand_normal_vec2(0.0, BALL_V)
        }
    }

    fn spawn_balls(loc: Vec2) -> Vec<Ball> {
        (0..BALL_COUNT).map(|_| {Ball::new(loc)}).collect()
    }

}

impl Cell {
    fn new(i: usize) -> Cell {
        let row: usize = i%NUM_CELLS;
        let column: usize = i/NUM_CELLS;
        
        let delta: f32 = WINDOW_SIZE as f32 / NUM_CELLS as f32;
        let min_coord: f32 = -1.0*WINDOW_SIZE as f32/2.0 + delta/4.0;
        let coord: Vec2 = vec2(
            min_coord+row as f32*delta, 
            min_coord+column as f32*delta
        );
        let polygon: Poly = Poly {
            p1: coord + vec2(0.0, delta/2.0),
            p2: coord ,
            p3: coord + vec2(delta/2.0, 0.0),
            p4: coord + vec2(delta/2.0, delta/2.0),
            color: Rgba::new(0.05490196, 0.61176471, 0.36078431, 0.65)
        };

        let random_balls: Vec<Ball> = Ball::spawn_balls(
            coord + vec2(delta/4.0, delta/4.0)
        );
        Cell {
            poly: polygon,
            balls: random_balls
        }
    }

    fn wall_check(&mut self) {
        for ball in &mut self.balls {
            

            if ball.loc.x < self.poly.p2.x || ball.loc.x > self.poly.p4.x {
                ball.v.x *= -1.0;

            }
            if ball.loc.y < self.poly.p2.y || ball.loc.y > self.poly.p4.y  {
                ball.v.y *= -1.0;
            }
        }
    }

}

struct Model {
    cells: Vec<Cell>
}

impl Model {
    fn new() -> Model {
        let cells: Vec<Cell> = (0..NUM_CELLS*NUM_CELLS).map(
            |i| {Cell::new(i)}
        ).collect();
        Model {cells}
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .key_released(key_released)
        .build()
        .unwrap();
    
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for cell in &mut model.cells {
        cell.wall_check();
        for ball in &mut cell.balls {
            ball.loc += ball.v*DT
        }
        
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.rect()
        .wh(app.window_rect().wh())
        .rgba(0.18431373, 0.19215686, 0.29019608, ALPHA);
    }
   
    let time = app.time;
    draw_cells(&draw, &model, time);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_cells(draw: &Draw, model: &Model, _time: f32) {
    for cell in &model.cells {
        draw.quad()
            .color(cell.poly.color)
            .points(cell.poly.p1, cell.poly.p2, cell.poly.p3, cell.poly.p4);
        for ball in &cell.balls {
            draw.ellipse()
                .xy(ball.loc)
                .radius(BALL_SIZE)
                .color(WHITE);

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