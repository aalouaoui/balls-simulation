use macroquad::{
    color::hsl_to_rgb,
    prelude::{draw_poly, draw_poly_lines, screen_height, screen_width, vec2, Color, Vec2},
};
use rand::Rng;

pub struct Ball {
    x: f32,
    y: f32,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
    sides: u8,
    color: Color,
    fill: bool,
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
            fill: false,
        }
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(0.05..0.1) * screen_height().min(screen_width());
        let x = rng.gen_range(radius..screen_width() - radius);
        let y = rng.gen_range(radius..screen_height() - radius);
        let color = hsl_to_rgb(rng.gen_range(0.0..1.0), 0.5, 0.5);
        Self::new(vec2(x, y), radius, color)
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.velocity.x * dt;
        self.y += self.velocity.y * dt;

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;
        self.handle_bound_collision();
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

    pub fn collide_with(&self, other: &Self) -> bool {
        vec2(self.x, self.y).distance(vec2(other.x, other.y)) < self.radius + other.radius
    }

    pub fn handle_balls_collision(balls: &mut Vec<Ball>) {
        balls.iter_mut().for_each(|ball| ball.fill = false);
        for i in 0..balls.len() - 1 {
            for j in i + 1..balls.len() {
                if balls[i].collide_with(&balls[j]) {
                    balls[i].fill = true;
                    balls[j].fill = true;
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.x,
            self.y,
            self.sides,
            self.radius,
            0.0,
            2.0,
            self.color,
        );
        if self.fill {
            draw_poly(self.x, self.y, self.sides, self.radius, 0.0, self.color);
        }
    }
}
