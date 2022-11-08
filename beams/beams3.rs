use csv::Reader;
use nalgebra::geometry::{Point3, Rotation3};
use nalgebra::norm;
use nannou::prelude::*;
use serde::Deserialize;

use nannou_egui::{self, egui, Egui};
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use std::collections::HashMap;
use std::f32::{self, consts::PI};

const WINDOW_SIZE: u32 = 1000;
const SCALE: f32 = 0.3;
const SPHERE_SIZE: f32 = WINDOW_SIZE as f32 * SCALE;
const N_BORBS: usize = 150;
const BORB_SPEED: f32 = 0.02;
const BREAK_COUNT: usize = 5;

const PITCH_SPEED: f32 = 0.02;

// Gui: https://github.com/nannou-org/nannou/blob/master/examples/ui/egui/circle_packing.rs

enum BorbMode {
    RAILS,
    FREE,
    CORE,
}

struct Node {
    pos: Point3<f32>,
}

impl Node {
    fn fade(&self) -> f32 {
        //(self.pos.z+SPHERE_SIZE/2.0)/(SPHERE_SIZE)
        0.5 + (self.pos.z - WINDOW_SIZE as f32 / 2.0) / (WINDOW_SIZE as f32)
    }
}

#[derive(Debug, Deserialize)]
struct Edge {
    src: usize,
    dest: usize,
    hop_count: usize,
    free: bool,
}

struct Borb {
    pos: Point3<f32>,
    dest_pos: Point3<f32>,
    src: usize,
    dest: usize,
    progress: f32,
    mode: BorbMode,
    gaussian: Normal,
    color: (f32, f32, f32),
}

impl Borb {
    fn spawn_random(nodes: &Vec<Node>, neighbors: &Vec<Vec<usize>>) -> Self {
        let src = rand::thread_rng().gen_range(0, nodes.len());
        let options = &neighbors[src];
        let index: usize = rand::thread_rng().gen_range(0, options.len());
        let dest = options[index];

        Self {
            pos: nodes[src].pos,
            dest_pos: nodes[dest].pos,
            src: src,
            dest: dest,
            progress: 0.0,
            mode: BorbMode::RAILS,
            gaussian: Normal::new(0.0, 10.0),
            color: (1.0, 0.1, 0.1),
        }
    }
    fn hop(&mut self, nodes: &Vec<Node>, options: &Vec<Vec<usize>>) {
        if options[self.dest].len() > 0 {
            self.src = self.dest;
            self.pos = self.dest_pos;
            let index = rand::thread_rng().gen_range(0, options[self.dest].len());
            let dest = options[self.dest][index];
            self.dest = dest;
            self.dest_pos = nodes[dest].pos;
            self.progress = 0.0;
        } else {
            self.progress = 0.0;
            self.mode = BorbMode::FREE;
            self.dest_pos = Point3::new(0.0, 0.0, 0.0);
        }
    }

    fn step(&mut self) {
        self.progress += BORB_SPEED;
        let offset = Point3::new(0.0, 0.0, 0.0);
        match self.mode {
            BorbMode::FREE => {
                let x = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                let y = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                let z = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                self.pos = self.pos + self.progress * (self.dest_pos - self.pos);
                // self.pos.x += x;
                // self.pos.y += y;
                // self.pos.z += z;
            }
            BorbMode::RAILS => {
                self.pos = self.pos + self.progress * (self.dest_pos - self.pos);
            }
            BorbMode::CORE => {
                let dx = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                let dy = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                let dz = self.gaussian.sample(&mut rand::thread_rng()) as f32;
                let dist = (self.pos.x.powi(2) + self.pos.y.powi(2) + self.pos.z.powi(2)).powf(0.5);
                self.pos.x -= self.pos.x - self.pos.x / dist + dx;
                self.pos.y -= self.pos.y - self.pos.y / dist + dy;
                self.pos.z -= self.pos.z - self.pos.z / dist + dz;
            }
        }
    }

    fn fade(&self) -> f32 {
        0.5 + (self.pos.z - SPHERE_SIZE / 2.0) / (SPHERE_SIZE)
    }

    fn size(&self) -> f32 {
        2.0 + 25.0 * (self.progress - 0.25).abs()
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Angles {
    roll: f32,
    pitch: f32,
    yaw: f32,
}
struct Model {
    nodes: Vec<Node>,
    edges: HashMap<(usize, usize), Edge>,
    borbs: Vec<Borb>,
    angles: Angles,
    egui: Egui,
    neighbors: Vec<Vec<usize>>,
}

impl Model {
    fn new(nodes: Vec<Node>, edges: HashMap<(usize, usize), Edge>, egui: Egui) -> Self {
        let angles = Angles {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        };
        let mut neighbors: Vec<Vec<usize>> = vec![Vec::<usize>::new(); nodes.len()];
        for ((src, dest), _) in edges.iter() {
            neighbors[*src].push(*dest);
            neighbors[*dest].push(*src);
        }

        let mut borbs: Vec<Borb> = Vec::new();
        for _ in 0..N_BORBS {
            borbs.push(Borb::spawn_random(&nodes, &neighbors));
        }

        Model {
            nodes,
            edges,
            borbs,
            angles,
            egui,
            neighbors,
        }
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let pos_f: String = String::from("beams/graphs/50_node/graph_positions.csv");
    let edge_f: String = String::from("beams/graphs/50_node/graph_edges.csv");
    let (nodes, edges) = read_graph(pos_f, edge_f);

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model::new(nodes, edges, egui)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let Model {
        ref mut nodes,
        ref mut edges,
        ref mut borbs,
        ref mut angles,
        ref mut neighbors,
        ..
    } = *model;

    // // EGUI
    // let ctx = model.egui.begin_frame();
    // let changed_axis = vec![Cell::new(false); 3];
    // egui::Window::new("Orientation").show(&ctx, |ui| {
    //     changed_axis[0].set(ui
    //         .add(egui::Slider::new(&mut angles.roll, -PI..=PI).text("roll"))
    //         .changed());
    //     changed_axis[1].set(ui
    //         .add(egui::Slider::new(&mut angles.pitch, -PI/2.0..=PI/2.0).text("pitch"))
    //         .changed());
    //     changed_axis[2].set(ui
    //         .add(egui::Slider::new(&mut angles.yaw, 0.0..=2.0*PI).text("yaw"))
    //         .changed());
    // });

    // Rotating points

    let mut broken_edges: Vec<(usize, usize)> = Vec::new();
    for (key, edge) in edges.iter() {
        if edge.hop_count >= BREAK_COUNT {
            broken_edges.push(*key);
        }
    }

    for (src, dest) in broken_edges {
        if let Some(pos) = neighbors[src].iter().position(|x| *x == dest) {
            neighbors[src].remove(pos);
        }
        if let Some(pos) = neighbors[dest].iter().position(|x| *x == src) {
            neighbors[dest].remove(pos);
        }

        let new_dest = rand::thread_rng().gen_range(0, nodes.len());
        let new_edge: Edge = Edge {
            src,
            dest: new_dest,
            hop_count: 0,
            free: true,
        };
        edges.insert((src, new_dest), new_edge);
        neighbors[src].push(new_dest);
        let new_dest = rand::thread_rng().gen_range(0, nodes.len());
        let new_edge: Edge = Edge {
            src: dest,
            dest: new_dest,
            hop_count: 0,
            free: true,
        };
        edges.insert((dest, new_dest), new_edge);
        neighbors[dest].push(new_dest);
        edges.remove(&(src, dest));
    }

    let r: Rotation3<f32> =
        Rotation3::from_euler_angles(angles.roll, angles.pitch + PITCH_SPEED, angles.yaw);

    for n in nodes.iter_mut() {
        n.pos = r * n.pos;
    }
    for b in borbs.iter_mut() {
        b.pos = r * b.pos;
        b.dest_pos = r * b.dest_pos;
    }

    // Step Objects
    for borb in borbs.iter_mut() {
        borb.step();
        if borb.progress >= 0.5 {
            borb.hop(&model.nodes, &model.neighbors);
            if let Some(e) = edges.get_mut(&(borb.src, borb.dest)) {
                e.hop_count += 1;
            }
            if let Some(e) = edges.get_mut(&(borb.dest, borb.src)) {
                e.hop_count += 1;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgba(0.0, 0.0, 0.0, 0.75);
    draw_model(&draw, &model);
    draw.to_frame(app, &frame).unwrap();
    //model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_model(draw: &Draw, model: &Model) {
    // Drawing nodes
    // for node in model.nodes.iter() {
    //     draw.ellipse()
    //         .x_y_z(node.pos.x, node.pos.y, node.pos.z)
    //         .roll(model.angles.roll)
    //         .pitch(model.angles.pitch)
    //         .yaw(model.angles.yaw)
    //         .radius(8.0)
    //         .rgba(0.10588235, 0.56078431, 0.43921569, node.fade());
    // }

    // Drawing edges
    for ((src, dest), e) in model.edges.iter() {
        let n1 = &model.nodes[*src];
        let n2 = &model.nodes[*dest];

        let fade: f32 = (n1.fade() + n2.fade()) / 2.0;
        let rc = e.hop_count as f32 / BREAK_COUNT as f32;
        if !e.free {
            draw.line()
                .start(vec2(n1.pos.x, n1.pos.y))
                .end(vec2(n2.pos.x, n2.pos.y))
                .weight(3.0)
                .rgba(rc, 0.5 - rc, 0.5 - rc, fade);
        }
    }

    for borb in model.borbs.iter() {
        draw.ellipse()
            .x_y_z(borb.pos.x, borb.pos.y, borb.pos.z)
            .radius(borb.size())
            .rgba(borb.color.0, borb.color.1, borb.color.2, borb.fade());
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

#[derive(Debug, Deserialize)]
struct NodeReader {
    id: usize,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Deserialize)]
struct EdgeReader {
    src: usize,
    dest: usize,
}

fn read_graph(pos_file: String, edge_file: String) -> (Vec<Node>, HashMap<(usize, usize), Edge>) {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: HashMap<(usize, usize), Edge> = HashMap::new();
    let mut rdr = Reader::from_path(pos_file).unwrap();
    for result in rdr.deserialize() {
        let n: NodeReader = result.unwrap();
        let node: Node = Node {
            pos: SPHERE_SIZE * Point3::new(n.x, n.y, n.z),
        };
        nodes.push(node);
    }
    let mut rdr = Reader::from_path(edge_file).unwrap();
    for result in rdr.deserialize() {
        let e: EdgeReader = result.unwrap();
        let edge: Edge = Edge {
            src: e.src,
            dest: e.dest,
            hop_count: 0,
            free: false,
        };
        edges.insert((edge.src, edge.dest), edge);
    }

    (nodes, edges)
}
