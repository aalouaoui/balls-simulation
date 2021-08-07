use macroquad::prelude::*;

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
    let mut balls: Vec<Ball> = vec![];
    let mut sw = screen_width();
    let mut sh = screen_height();

    loop {
        let dt = get_frame_time();
        if screen_width().ne(&sw) || screen_height().ne(&sh) || balls.len() == 0 {
            balls.clear();
            'outer: for _ in 0..60 {
                let mut counter = 0;
                balls.push(loop {
                    let new_ball = Ball::new_random();
                    if !balls.iter().any(|ball| ball.collide_with(&new_ball)) {
                        break new_ball;
                    }
                    counter += 1;
                    if counter > 1000 {
                        break 'outer;
                    }
                });
            }
            sw = screen_width();
            sh = screen_height();
        }

        balls.iter_mut().for_each(|ball| ball.update(dt));
        Ball::handle_balls_collision(&mut balls);
        balls.iter().for_each(|ball| ball.draw());

        next_frame().await
    }
}
