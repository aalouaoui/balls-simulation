use macroquad::{color::hsl_to_rgb, prelude::*};

pub struct Ball {
    x: f32,
    y: f32,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
    sides: u8,
    color: Color,
}

impl Ball {
    pub fn new(pos: Vec2, radius: f32, color: Color) -> Self {
        Ball {
            x: pos.x,
            y: pos.y,
            velocity: vec2(200.0, -300.0),
            acceleration: vec2(0.0, 50.0),
            radius,
            sides: (radius / 2.0).clamp(20.0, 255.0) as u8,
            color,
        }
    }

    pub fn new_random() -> Self {
        let radius = rand::gen_range(0.05, 0.1) * screen_height().min(screen_width());
        let x = rand::gen_range(radius, screen_width() - radius);
        let y = rand::gen_range(radius, screen_height() - radius);
        let color = hsl_to_rgb(rand::gen_range(0.0, 1.0), 0.5, 0.5);
        Self::new(vec2(x, y), radius, color)
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.velocity.x * dt;
        self.y += self.velocity.y * dt;

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;
        self.handle_bound_collision();
        self.draw();
    }

    fn handle_bound_collision(&mut self) {
        let max_x = screen_width() - self.radius;
        let max_y = screen_height() - self.radius;
        if self.x < self.radius || self.x > max_x {
            self.velocity.x = -self.velocity.x;
        }
        if self.y < self.radius || self.y > max_y {
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.x,
            self.y,
            self.sides,
            self.radius,
            0.0,
            1.0,
            self.color,
        );
    }
}
