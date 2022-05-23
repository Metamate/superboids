mod boid;
use boid::Boid;
use nannou::prelude::*;

const NO_BOIDS: u16 = 100;

const ALIGNMENT: f32 = 1.;
const COHESION: f32 = 0.05;
const SEPARATION: f32 = 1.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    boids: Vec<Boid>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut boids = Vec::new();

    let (left, right, bottom, top) = app.window_rect().l_r_b_t();
    for _n in 0..NO_BOIDS {
        let boid = Boid::new(
            random_range(left, right),
            random_range(bottom, top),
            15.,
            15.,
        );
        boids.push(boid);
    }

    Model { _window, boids }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let (left, right, bottom, top) = app.window_rect().l_r_b_t();

    for i in 0..model.boids.len() {
        let local_boids = model.boids[i].local_boids(&model.boids);
        let alignment = model.boids[i].alignment(&local_boids);
        let cohesion = model.boids[i].cohesion(&local_boids);
        let separation = model.boids[i].separation(&local_boids);

        model.boids[i].acceleration +=
            alignment * ALIGNMENT + cohesion * COHESION + separation * SEPARATION;
        model.boids[i].update();
        model.boids[i].contain(left, right, bottom, top);
    }
    //println!("{:?}", 1. / _update.since_last.as_secs_f64());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for boid in _model.boids.iter() {
        boid.show(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
