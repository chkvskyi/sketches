use nannou::prelude::*;

const Z: u32 = 1024;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(Z, Z)
        .run();
}

struct Model {
    ts: f32,
}

fn model(_app: &App) -> Model {
    Model { ts: 0.0 }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.ts = (update.since_start.as_millis() as f32) / 1000.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let start = -(Z as i32) / 2;
    let end = (Z as i32) / 2;
    let points = (start..end).map(|i| {
        let x = i as f32;
        let offset = (Z as f32) / 2.0;
        let yx = x + offset;
        let y = (yx / 2.0) * (yx / 30.0 - model.ts).sin();
        (pt2(x, y), WHITE)
    });
    draw.polyline().weight(3.0).points_colored(points);
    draw.to_frame(app, &frame).unwrap();
}
