mod boid;
use boid::Boid;
use nannou::prelude::*;

const NO_OF_BOIDS: u16 = 1000;

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

    let boundary = app.window_rect().l_r_b_t();
    for _n in 0..NO_OF_BOIDS {
        let random = (
            random_range(boundary.0, boundary.1),
            random_range(boundary.2, boundary.3),
        );
        let boid = Boid::new(random.0, random.1);
        boids.push(boid);
    }

    Model { _window, boids }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boids = model.boids.clone();
    for boid in model.boids.iter_mut() {
        boid.update(app, boids.clone());
    }
    println!("{:?}", 1. / _update.since_last.as_secs_f64());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for boid in _model.boids.iter() {
        boid.show(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
