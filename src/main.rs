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

    for _n in 0..NO_OF_BOIDS {
        boids.push(spawn_boid(app));
    }

    Model { _window, boids }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    for boid in _model.boids.iter_mut() {
        boid.position += boid.velocity;
        boid.contain(app);
    }
    println!("{:?}", 1. / _update.since_last.as_secs_f64());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    for boid in _model.boids.iter() {
        boid.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn spawn_boid(app: &App) -> Boid {
    let boundary = app.window_rect().l_r_b_t();
    let random = (
        random_range(boundary.0, boundary.1),
        random_range(boundary.2, boundary.3),
    );
    let boid = Boid::new(random.0, random.1);
    boid
}
