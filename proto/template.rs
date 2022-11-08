use nannou::prelude::*;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    
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
    todo!();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    todo!();
   
    let time = app.time;
    draw_model(&draw, &model, time);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_model(draw: &Draw, model: &Model, time: f32) {
    todo!();
}
fn key_released(app: &App, _model: &mut Model, key: Key) {
    todo!();
}