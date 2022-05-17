use std::ops::Sub;

use nannou::prelude::*;

use crate::Model;

#[derive(PartialEq, Copy, Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    acceleration: Vec2,
    color: Srgb<u8>,
}

impl Boid {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            position: Vec2::new(pos_x, pos_y),
            velocity: Vec2::new(random_f32() - 0.5, random_f32() - 0.5),
            acceleration: Vec2::default(),
            color: Srgb::new(random(), random(), random()),
        }
    }

    pub fn update(&mut self, app: &App, boids: Vec<Boid>) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.flock(boids);
        self.contain(app);
    }

    pub fn show(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .w_h(10., 10.)
            .x_y(self.position.x, self.position.y);
    }

    fn contain(&mut self, app: &App) {
        let boundary = app.window_rect().l_r_b_t();
        if self.position.x > boundary.1 {
            self.position.x = boundary.0;
        } else if self.position.x < boundary.0 {
            self.position.x = boundary.1;
        }
        if self.position.y > boundary.3 {
            self.position.y = boundary.2;
        } else if self.position.y < boundary.2 {
            self.position.y = boundary.3;
        }
    }

    pub fn flock(&mut self, boids: Vec<Boid>) {
        let alignment = self.align(boids);
        self.acceleration = alignment;
    }

    fn align(&mut self, boids: Vec<Boid>) -> Vec2 {
        let perception_radius = 10.;
        let local_boids: Vec<Boid> = boids
            .into_iter()
            .filter(|boid| {
                self.position.distance(boid.position) < perception_radius && boid != self
            })
            .collect();

        if local_boids.len() > 0 {
            let steering = local_boids
                .iter()
                .fold(Vec2::default(), |sum, boid| sum + boid.velocity)
                / local_boids.len() as f32;
            self.velocity -= steering;
            return steering;
        } else {
            Vec2::ZERO
        }
    }
}
