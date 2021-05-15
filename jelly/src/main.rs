use nannou::prelude::*;
pub mod quadtree;

use crate::quadtree::{Boundary, QuadTree, TreePoint};

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
    pub tree: QuadTree,
}

fn model(_app: &App) -> Model {
    // let c1 = rgb(245. / 256., 157. / 256., 140. / 256.);
    let c2 = rgb(245. / 256., 239. / 256., 188. / 256.);
    let c3 = rgb(140. / 256., 245. / 256., 203. / 256.);
    let c4 = rgb(172. / 256., 152. / 256., 245. / 256.);

    let mut balls: Vec<Ball> = Vec::new();
    for _b in 0..20000 {
        let angle = random_range::<f32>(-180., 180.);
        let d = random_range::<f32>(Z / 8. + CR / 2., H - 60.);
        let r = random_f32();
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
    Model {
        balls: balls,
        tree: QuadTree::new(
            Boundary {
                w: Z,
                h: Z,
                x: 0.,
                y: 0.,
            },
            2,
        ),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.tree = QuadTree::new(
        Boundary {
            w: Z,
            h: Z,
            x: 0.,
            y: 0.,
        },
        2,
    );
    for (i, mut b) in model.balls.iter_mut().enumerate() {
        b.pos = b.pos + b.vel;
        let d = b.pos.magnitude();
        if d > H - 50. - CR / 2. {
            b.vel = b.vel + b.pos.normalize() * 2. * b.vel.dot(-b.pos.normalize());
        }

        if d < (Z / 8.) + CR / 2. {
            b.vel = b.vel - b.pos.normalize() * 2. * b.vel.dot(b.pos.normalize());
        }

        model
            .tree
            .insert(TreePoint::new(b.pos.x, b.pos.y, i as u32))
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let inner_c = rgb(7. / 256., 30. / 256., 62. / 256.);
    let point_c = rgb(168. / 256., 108. / 256., 96. / 256.);
    let out_c = rgb(255. / 256., 206. / 256., 196. / 256.);
    // let point_c = rgb(79. / 256., 168. / 256., 173. / 256.);
    frame.clear(inner_c);
    let draw = app.draw();
    draw.ellipse().w(Z - 100.).h(Z - 100.).color(inner_c);
    // draw.ellipse().w(Z / 4.).h(Z / 4.).color(out_c);

    let inner_r: f32 = Z / 4.0;
    let count: u32 = 500;
    let angle: f32 = 2.0 * PI / count as f32;
    // for i in 0..count {
    // let x = f32::cos((i as f32) * angle) * inner_r;
    // let y = f32::sin((i as f32) * angle) * inner_r;
    // draw.ellipse().w(1.0).h(1.0).x(x).y(y).color(point_c);
    // }
    // for b in model.balls.iter() {
    //     draw.ellipse().w(CR).h(CR).x(b.pos.x).y(b.pos.y).color(b.c);
    // }
    model.tree.render(&draw);
    draw.to_frame(app, &frame).unwrap();
}
