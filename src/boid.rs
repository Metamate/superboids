use std::ops::{Div, Sub};

use nannou::prelude::*;

const PERCEPTION_RADIUS: f32 = 50.;

#[derive(PartialEq, Copy, Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    max_force: f32,
    max_velocity: f32,
    min_velocity: f32,
    width: f32,
    height: f32,
    color: Srgb<u8>,
}

impl Boid {
    pub fn new(pos_x: f32, pos_y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(pos_x, pos_y),
            velocity: Vec2::new(random_f32() - 0.5, random_f32() - 0.5),
            acceleration: Vec2::new(random_f32() - 0.5, random_f32() - 0.5),
            max_force: 0.5,
            max_velocity: 5.,
            min_velocity: 1.5,
            width,
            height,
            color: Srgb::new(random(), random(), random()),
        }
    }

    pub fn update(&mut self) {
        self.acceleration = self.acceleration.clamp_length_max(self.max_force);
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = self
            .velocity
            .clamp_length(self.min_velocity, self.max_velocity);
        self.acceleration = Vec2::ZERO;
    }

    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .x_y(self.position.x, self.position.y)
            .w_h(self.width, self.height)
            .rotate(self.velocity.angle())
            .color(self.color);
    }

    pub fn local_boids<'a>(&self, boids: &'a Vec<Boid>) -> Vec<&'a Boid> {
        boids
            .into_iter()
            .filter(|boid| {
                *boid != self && self.position.distance(boid.position) < PERCEPTION_RADIUS
            })
            .collect()
    }

    pub fn contain(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        if self.position.x > right {
            self.position.x = left;
        } else if self.position.x < left {
            self.position.x = right;
        }
        if self.position.y > top {
            self.position.y = bottom;
        } else if self.position.y < bottom {
            self.position.y = top;
        }
    }

    pub fn alignment(&self, local_boids: &Vec<&Boid>) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids
            .iter()
            .fold(Vec2::ZERO, |sum, boid| sum + boid.velocity)
            .div(len as f32)
            .sub(self.velocity)
    }

    pub fn cohesion(&self, local_boids: &Vec<&Boid>) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids
            .iter()
            .fold(Vec2::ZERO, |sum, boid| sum + boid.position)
            .div(len as f32)
            .sub(self.position)
    }

    pub fn separation(&self, local_boids: &Vec<&Boid>) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids.iter().fold(Vec2::ZERO, |sum, boid| {
            let distance = self.position.distance(boid.position);
            let inverse_magnitude = if distance != 0. { distance } else { f32::MIN };

            sum - boid.position.sub(self.position).div(inverse_magnitude)
        })
    }
}
