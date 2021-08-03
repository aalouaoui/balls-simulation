use macroquad::{hash, prelude::*, ui::root_ui};

mod ball;
use ball::Ball;

fn window_conf() -> Conf {
    Conf {
        window_title: "Balls Simulation".to_owned(),
        sample_count: 2,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut balls = vec![];
    let mut sw = screen_width();
    let mut sh = screen_height();
    let mut last_render_time = get_time();
    loop {
        let dt = (get_time() - last_render_time) as f32;
        last_render_time = get_time();

        if screen_width().ne(&sw) || screen_height().ne(&sh) || balls.len() == 0 {
            balls = (0..2).map(|_| Ball::new_random()).collect();
            sw = screen_width();
            sh = screen_height();
        }

        root_ui().window(hash!(), Vec2::ZERO, vec2(150.0, 100.0), |ui| {
            ui.label(None, &format!("FPS: {}", get_fps()));
            ui.label(None, &format!("dt: {}", dt));
        });

        balls.iter_mut().for_each(|ball| ball.update(dt));

        next_frame().await
    }
}