use nannou::prelude::*;

const Z: f32 = 900.0;
const H: f32 = Z / 2.0;
const CR: f32 = 4.;
const SPEED: f32 = 0.5;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(Z as u32, Z as u32)
        .run();
}

#[derive(Clone, Copy)]
struct Ball {
    vel: Vector2,
    pos: Vector2,
    c: Rgb,
}

struct Model {
    balls: Vec<Ball>,
}

fn model(_app: &App) -> Model {
    let c2 = rgb(245. / 256., 239. / 256., 188. / 256.);
    let c3 = rgb(140. / 256., 245. / 256., 203. / 256.);
    let c4 = rgb(172. / 256., 152. / 256., 245. / 256.);

    let mut balls: Vec<Ball> = Vec::new();
    for _b in 0..2000 {
        let angle = random_range::<f32>(-180., 180.);
        let d = random_range::<f32>(Z / 8. + CR / 2., H - 60.);
        let r = random::<f32>();
        let c = if r < 0.33 {
            c2
        } else if r < 0.66 {
            c3
        } else {
            c4
        };
        balls.push(Ball {
            vel: vec2(
                random_range::<f32>(-SPEED, SPEED),
                random_range::<f32>(-SPEED, SPEED),
            ),
            pos: vec2(angle.sin() * d, angle.cos() * d),
            c: c,
        });
    }
    Model { balls: balls }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for mut b in model.balls.iter_mut() {
        b.pos = b.pos + b.vel;
        let d = b.pos.magnitude();
        if d > H - 50. - CR / 2. {
            b.vel = b.vel + b.pos.normalize() * 2. * b.vel.dot(-b.pos.normalize());
        }

        if d < (Z / 8.) + CR / 2. {
            b.vel = b.vel - b.pos.normalize() * 2. * b.vel.dot(b.pos.normalize());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let inner_c = rgb(245. / 256., 178. / 256., 165. / 256.);
    frame.clear(inner_c);
    let draw = app.draw();
    draw.ellipse().w(Z - 100.).h(Z - 100.).color(inner_c);

    for b in model.balls.iter() {
        draw.ellipse().w(CR).h(CR).x(b.pos.x).y(b.pos.y).color(b.c);
    }
    draw.to_frame(app, &frame).unwrap();
}
