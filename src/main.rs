use std::time::{Duration, Instant};

use minifb::{Key, Window, WindowOptions};

const GRID_X: usize = 630;
const GRID_Y: usize = 330;

const DEFAULT_ENERGY_LIGHT: i32 = 16;

const TARGET_FPS: u64 = 60;

const WINDOW_X: usize = 630;
const WINDOW_Y: usize = 330;

fn main() {
    fastrand::seed(4);

    // create app instance
    let mut simulation = rustymold::Simulation::new(GRID_X, GRID_Y, DEFAULT_ENERGY_LIGHT);

    // create some molds
    for dx in (10..GRID_X).step_by(20) {
        for dy in (10..GRID_Y).step_by(20) {
            simulation.generate_mold(dx, dy);
        }
    }

    // create window
    let options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: minifb::Scale::X2, // x2 zoom level by default
        scale_mode: minifb::ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };

    let mut window = Window::new("rusty-mold", WINDOW_X, WINDOW_Y, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit frame rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(
        1_000_000 / TARGET_FPS,
    )));
    window.set_background_color(0, 0, 0);

    // frame buffer
    let mut buffer: Vec<u32> = vec![0; GRID_X * GRID_Y];

    // current window size
    let window_x: usize = WINDOW_X;
    let window_y: usize = WINDOW_Y;

    let mut last_frame_time: Instant;

    // main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        last_frame_time = Instant::now();

        for _ in 0..1 {
            simulation.update();
        }
        let elapsed: Duration = last_frame_time - Instant::now();
        print!("fps: {:.0}", 1. / elapsed.as_secs_f64());

        simulation.render(&mut buffer, window_x, window_y);

        window
            .update_with_buffer(&buffer, window_x, window_y)
            .unwrap();
    }
}
