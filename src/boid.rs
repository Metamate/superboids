use nannou::prelude::*;

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

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .w_h(10., 10.)
            .x_y(self.position.x, self.position.y);
    }

    pub fn contain(&mut self, app: &App) {
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
}
