use nannou::prelude::*;
use serde::Deserialize;
use csv::Reader;
use nalgebra::geometry::{Point3, Rotation3};
use nannou_egui::{self, egui, Egui};
use std::f32::{consts::PI, self};
use std::cell::Cell;

const WINDOW_SIZE: u32 = 1000;
const SCALE: f32 = 0.3;
const SPHERE_SIZE: f32 = WINDOW_SIZE as f32 * SCALE;


// Gui: https://github.com/nannou-org/nannou/blob/master/examples/ui/egui/circle_packing.rs


struct Node {
    pos: Point3<f32>
}

impl Node {
    fn fade(&self) -> f32 {
        0.5-(self.pos.z-SPHERE_SIZE/2.0)/(SPHERE_SIZE)
    }
}
#[derive(Debug, Deserialize)]
struct Edge {
    src: usize,
    dest: usize
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Angles {
    roll: f32,
    pitch: f32,
    yaw: f32
}
struct Model {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    angles: Angles,
    egui: Egui,
}

fn rotate_points(nodes: &mut Vec<Node>, angles: &mut Angles) {
    let r: Rotation3<f32> = Rotation3::from_euler_angles(
        angles.roll, angles.pitch, angles.yaw
    );

    for n in nodes.iter_mut() {
        n.pos = r*n.pos;
    }
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    
    let pos_f: String = String::from("beams/graphs/50_node/graph_positions.csv");
    let edge_f: String = String::from("beams/graphs/50_node/graph_edges.csv");
    let (nodes, edges) = read_graph(pos_f, edge_f);

    let angles = Angles {roll:0.0, pitch:0.0, yaw:0.0};
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {nodes, edges, angles, egui}
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let Model {
        ref mut nodes,
        ref mut angles,
        ..
    } = *model;

    let ctx = model.egui.begin_frame();
    let changed_axis = vec![Cell::new(false); 3];

    egui::Window::new("Orientation").show(&ctx, |ui| {
        changed_axis[0].set(ui
            .add(egui::Slider::new(&mut angles.roll, -PI..=PI).text("roll"))
            .changed());
        changed_axis[1].set(ui
            .add(egui::Slider::new(&mut angles.pitch, -PI/2.0..=PI/2.0).text("pitch"))
            .changed());
        changed_axis[2].set(ui
            .add(egui::Slider::new(&mut angles.yaw, 0.0..=2.0*PI).text("yaw"))
            .changed());
    });

    if changed_axis.iter().any(|angle| angle.get() == true) {
        rotate_points(nodes, angles);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_model(&draw, &model);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_model(draw: &Draw, model: &Model) {
    for node in model.nodes.iter() {

        draw.ellipse()
            .x_y_z(node.pos.x, node.pos.y, node.pos.z)
            .radius(5.0)
            .rgba(1.0,0.0,0.0, node.fade());
    }
    
    for edge in model.edges.iter() {
        let n1 = &model.nodes[edge.src];
        let n2 = &model.nodes[edge.dest];

        let fade: f32 = (n1.fade() + n2.fade())/2.0;
        draw.line()
            .start(vec2(n1.pos.x, n1.pos.y))
            .end(vec2(n2.pos.x, n2.pos.y))
            .weight(1.0)
            .rgba(0.6,0.0,0.0, fade);
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
    z: f32
}

fn read_graph(pos_file: String, edge_file: String) -> (Vec<Node>, Vec<Edge>) {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut rdr = Reader::from_path(pos_file).unwrap();
    for result in rdr.deserialize() {
        let n: NodeReader = result.unwrap();
        let node: Node = Node { 
            pos: SPHERE_SIZE*Point3::new(n.x, n.y, n.z) 
        };
        nodes.push(node);
    }
    let mut rdr = Reader::from_path(edge_file).unwrap();
    for result in rdr.deserialize() {
        let edge: Edge = result.unwrap();
        edges.push(edge);
    }

    (nodes, edges)
}