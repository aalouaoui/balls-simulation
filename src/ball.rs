use macroquad::{
    color::hsl_to_rgb,
    prelude::{draw_poly, draw_poly_lines, screen_height, screen_width, vec2, Color, Vec2},
};
use rand::Rng;

pub struct Ball {
    pos: Vec2,
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
            pos,
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
        self.pos += self.velocity * dt;
        self.velocity += self.acceleration * dt;
        self.handle_bound_collision();
    }

    fn handle_bound_collision(&mut self) {
        let max_x = screen_width() - self.radius;
        let max_y = screen_height() - self.radius;
        if self.pos.x <= self.radius {
            self.velocity.x = -self.velocity.x;
            self.pos.x = self.radius + 1.0;
        } else if self.pos.x >= max_x {
            self.velocity.x = -self.velocity.x;
            self.pos.x = max_x - 1.0;
        }
        if self.pos.y <= self.radius {
            self.velocity.y = -self.velocity.y;
            self.pos.y = self.radius + 1.0;
        } else if self.pos.y >= max_y {
            self.velocity.y = -self.velocity.y;
            self.pos.y = max_y - 1.0;
        }
    }

    pub fn outer_distance(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos) - self.radius - other.radius
    }

    pub fn collide_with(&self, other: &Self) -> bool {
        self.outer_distance(other) <= 0.0
    }

    fn get_inv_mass(&self) -> f32 {
        1.0 / self.radius
    }

    pub fn handle_balls_collision(balls: &mut Vec<Ball>) {
        balls.iter_mut().for_each(|ball| ball.fill = false);
        for i in 0..balls.len() - 1 {
            for j in i + 1..balls.len() {
                Ball::check_and_resolve_collision(balls, i, j);
            }
        }
    }

    pub fn check_and_resolve_collision(balls: &mut Vec<Ball>, i: usize, j: usize) {
        if !balls[i].collide_with(&balls[j]) {
            return;
        }
        let distance = balls[i].outer_distance(&balls[j]);
        let normal = (balls[i].pos - balls[j].pos).normalize();

        // Cheap trick to prevent balls from getting stuck on collision
        balls[i].pos -= 0.5 * distance * normal;
        balls[j].pos += 0.5 * distance * normal;

        // Stolen from
        // https://github.com/danielszabo88/mocorgo/blob/master/09%20-%20Mass%20and%20Elasticity/script.js
        let inv_mass_i = balls[i].get_inv_mass();
        let inv_mass_j = balls[j].get_inv_mass();

        let sep_vec = (balls[i].velocity - balls[j].velocity).dot(normal) * normal;
        let impulse_vec = 2.0 * sep_vec / (inv_mass_i + inv_mass_j);

        balls[i].velocity -= impulse_vec * inv_mass_i;
        balls[j].velocity += impulse_vec * inv_mass_j;
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            self.sides,
            self.radius,
            0.0,
            2.0,
            self.color,
        );
        if self.fill {
            draw_poly(
                self.pos.x,
                self.pos.y,
                self.sides,
                self.radius,
                0.0,
                self.color,
            );
        }
    }
}
