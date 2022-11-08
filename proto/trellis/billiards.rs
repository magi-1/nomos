use nannou::prelude::*;
use rand::distributions::{Distribution, Normal};
use std::collections::HashMap;

const WINDOW_SIZE: u32 = 1000;
const NUM_CELLS: usize = 3;
const BALL_COUNT: usize = 9;
const BALL_SIZE: f32 = 4.0;
const POCKET_SIZE: f32 = 10.0;
const NUM_PARTICLES: usize = 10;

const BALL_V: f32 = 20.0;
const ALPHA: f32 = 0.1;
const DT: f32 = 0.3;

// Opening scene there is a pool ball that flashes behind the squares with a flash
// Balls ordered in a triangle (maybe)
// If a ball goes in a pocket, we want to visualize it bellow the pool table as a colored ellipse

fn rand_normal_vec2(mu: f32, var: f32) -> Vec2 {
    let normal = Normal::new(mu as f64, var as f64);
    vec2(
        normal.sample(&mut rand::thread_rng()) as f32,
        normal.sample(&mut rand::thread_rng()) as f32,
    )
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Poly {
    points: Vec<Vec2>,
    color: Rgba,
}

#[derive(Copy, Clone)]
struct Ball {
    loc: Vec2,
    v: Vec2,
    color: Rgba,
}

struct Circle {
    loc: Vec2,
    color: Rgba,
    r: f32,
}

impl Circle {
    fn new(loc: Vec2, color: Rgba) -> Circle {
        Circle {
            loc: loc,
            color: color,
            r: 1.0,
        }
    }
}
struct Cell {
    poly: Poly,
    balls: HashMap<usize, Ball>,
    pocketed: HashMap<usize, Ball>,
    pocket_events: HashMap<usize, Circle>,
}

impl Ball {
    fn new(loc: Vec2, color: Rgba) -> Ball {
        let mut v: Vec2 = rand_normal_vec2(0.0, 1.0);
        let mag: f32 = (v.x * v.x + v.y * v.y).sqrt();
        v /= mag;
        v *= BALL_V;

        Ball {
            loc: loc,
            v: v,
            color: color,
        }
    }

    fn spawn_balls(loc: Vec2, colors: Vec<Rgba>) -> HashMap<usize, Ball> {
        let mut balls = HashMap::new();

        for i in 0..BALL_COUNT {
            balls.insert(i, Ball::new(loc, colors[i % colors.len()]));
        }
        balls
    }
}

impl Cell {
    fn new(i: usize) -> Cell {
        let row: usize = i % NUM_CELLS;
        let column: usize = i / NUM_CELLS;

        let delta_x: f32 = WINDOW_SIZE as f32 / NUM_CELLS as f32;
        let delta_y: f32 = WINDOW_SIZE as f32 / NUM_CELLS as f32;

        let min_x: f32 = -1.0 * WINDOW_SIZE as f32 / 2.0 + delta_x / 4.0;
        let min_y: f32 = -1.0 * WINDOW_SIZE as f32 / 2.0 + delta_y / 4.0;
        let coord: Vec2 = vec2(
            min_x + row as f32 * delta_x,
            min_y + column as f32 * delta_y,
        );

        let polygon: Poly = Poly {
            points: vec![
                coord + vec2(0.0, delta_y / 2.0),
                coord,
                coord + vec2(delta_x / 2.0, 0.0),
                coord + vec2(delta_x / 2.0, delta_y / 2.0),
            ],
            color: Rgba::new(0.05490196, 0.61176471, 0.36078431, 0.65),
        };

        let ball_colors: Vec<Rgba> = vec![
            Rgba::new(0.835, 0.125, 0.125, 1.0),
            Rgba::new(0.125, 0.235, 0.835, 1.0),
            Rgba::new(0.921, 0.839, 0.078, 1.0),
            Rgba::new(0.666, 0.078, 0.921, 1.0),
            Rgba::new(0.584, 0.352, 0.156, 1.0),
            Rgba::new(0.109, 0.588, 0.090, 1.0),
            Rgba::new(0.960, 0.505, 0.039, 1.0),
            Rgba::new(0.0, 0.0, 0.0, 1.0),
            Rgba::new(1.0, 1.0, 1.0, 1.0),
        ];

        let random_balls: HashMap<usize, Ball> =
            Ball::spawn_balls(coord + vec2(delta_x / 4.0, delta_y / 4.0), ball_colors);

        Cell {
            poly: polygon,
            balls: random_balls,
            pocketed: HashMap::new(),
            pocket_events: HashMap::new(),
        }
    }

    fn wall_check(&mut self) {
        for (_, ball) in &mut self.balls {
            if ball.loc.x < self.poly.points[1].x || ball.loc.x > self.poly.points[3].x {
                ball.v.x *= -1.0;
            }
            if ball.loc.y < self.poly.points[1].y || ball.loc.y > self.poly.points[3].y {
                ball.v.y *= -1.0;
            }
        }
    }

    fn pocket_check(&mut self) {
        for pocket in &self.poly.points {
            let mut delete_log: Vec<usize> = Vec::new();
            for (i, ball) in &self.balls {
                let delta = *pocket - ball.loc;
                let distance = (delta).dot(delta).sqrt();

                if distance < POCKET_SIZE / 2.0 {
                    let num_pocketed: usize = self.pocketed.len();
                    let mut b: Ball = *ball;
                    let pdelta: f32 = WINDOW_SIZE as f32 / NUM_CELLS as f32 / 20.0;
                    b.loc = self.poly.points[1]
                        + vec2(pdelta * num_pocketed as f32 + 17.0, -pdelta * 1.5);
                    self.pocketed.insert(*i, b);
                    delete_log.push(*i);
                    let c: Circle = Circle::new(*pocket, b.color);
                    self.pocket_events.insert(*i, c);
                }
            }

            for i in delete_log.iter() {
                self.balls.remove(i);
            }
        }
    }

    fn pocket_event_update(&mut self) {
        let mut keys_to_delete: Vec<usize> = Vec::new();
        for (i, circle) in &mut self.pocket_events {
            if circle.r >= 25.0 {
                keys_to_delete.push(*i);
            } else {
                circle.r += 1.0;
            }
        }

        for i in keys_to_delete.iter() {
            self.pocket_events.remove(i);
        }
    }
}

struct Model {
    cells: Vec<Cell>,
}

impl Model {
    fn new() -> Model {
        let cells: Vec<Cell> = (0..NUM_CELLS * NUM_CELLS).map(|i| Cell::new(i)).collect();
        Model { cells }
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
        cell.pocket_check();
        cell.pocket_event_update();
        for (_, ball) in &mut cell.balls {
            ball.loc += ball.v * DT;
            ball.v *= 0.995;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    //if frame.nth() == 0 {
    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(0.18431373, 0.19215686, 0.29019608, ALPHA);
    //}

    let time = app.time;
    draw_cells(&draw, &model, time);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_cells(draw: &Draw, model: &Model, _time: f32) {
    for cell in &model.cells {
        draw.quad().color(cell.poly.color).points(
            cell.poly.points[0],
            cell.poly.points[1],
            cell.poly.points[2],
            cell.poly.points[3],
        );

        for i in 0..9 {
            let pdelta: f32 = WINDOW_SIZE as f32 / NUM_CELLS as f32 / 20.0;
            draw.ellipse()
                .xy(cell.poly.points[1] + vec2(pdelta * i as f32 + 17.0, -pdelta * 1.5))
                .radius(BALL_SIZE * 1.5)
                .color(WHITE);
        }

        for (_, circle) in &cell.pocket_events {
            draw.ellipse()
                .xy(circle.loc)
                .radius(circle.r)
                .color(circle.color);
        }

        for p in &cell.poly.points {
            draw.ellipse().xy(*p).radius(POCKET_SIZE).color(BLACK);
        }

        for (_, ball) in &cell.balls {
            draw.ellipse()
                .xy(ball.loc)
                .radius(BALL_SIZE)
                .color(ball.color);
        }

        for (_, ball) in &cell.pocketed {
            draw.ellipse()
                .xy(ball.loc)
                .radius(BALL_SIZE)
                .color(ball.color);
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
