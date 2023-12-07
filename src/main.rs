use std::time::{Duration, Instant};

use minifb::{Key, MouseMode, Window, WindowOptions};

const GRID_X: usize = 630;
const GRID_Y: usize = 330;

const DEFAULT_ENERGY_LIGHT: i32 = 16;

// window defaults
const TARGET_FPS: u64 = 60;
const BUFFER_X: usize = GRID_X; // initial size of screen buffer - half the size of the window in pixels
const BUFFER_Y: usize = GRID_Y;
const ZOOM: usize = 1;
const MIN_ZOOM: usize = 1;
const MAX_ZOOM: usize = 16;

fn main() {
    // fastrand::seed(4);

    // create simulation instance
    let mut simulation = rustymold::Simulation::new(GRID_X, GRID_Y, DEFAULT_ENERGY_LIGHT);

    // create window
    let options = WindowOptions {
        borderless: false,
        title: true,
        resize: true,
        scale: minifb::Scale::X2, // x2 zoom level by default
        scale_mode: minifb::ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window = Window::new("rusty-mold", BUFFER_X, BUFFER_Y, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit frame rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(
        1_000_000 / TARGET_FPS,
    )));
    window.set_background_color(0, 0, 0);

    // current window state
    let mut buffer: Vec<u32> = vec![0; BUFFER_X * BUFFER_Y];
    let mut buffer_size: (usize, usize) = (BUFFER_X, BUFFER_Y);
    let mut zoom = ZOOM;
    // offset representing the amount of pixels that the simulation grid is panned
    let mut camera_position: (f32, f32) = (0.0, 0.0);

    let mut is_mouse_right_down: bool = false;
    let mut mouse_pan_start: (f32, f32) = camera_position;

    let mut last_frame_time = Instant::now();
    let mut average_fps: f64 = TARGET_FPS as f64;

    let mut is_running: bool = true;

    // main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // reshape frame buffer if the window was resized
        let new_window_size = window.get_size();
        let new_buffer_size = (new_window_size.0 / 2, new_window_size.1 / 2);
        if new_buffer_size != buffer_size {
            buffer_size = new_buffer_size;
            let new_buffer_length = buffer_size.0 * buffer_size.1;
            buffer.resize(new_buffer_length, 0)
        }

        // handle keyboard/mouse input
        // zoom when scroll wheel is used
        if let Some(scroll) = window.get_scroll_wheel() {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                if scroll.1 < 0. && zoom > MIN_ZOOM {
                    let zoom_ratio = 1. - 1. / zoom as f32;
                    zoom -= 1;

                    camera_position = (
                        (camera_position.0 + x) * zoom_ratio - x,
                        (camera_position.1 + y) * zoom_ratio - y,
                    );
                } else if scroll.1 > 0. && zoom < MAX_ZOOM {
                    let zoom_ratio = 1. + 1. / zoom as f32;
                    zoom += 1;

                    camera_position = (
                        (camera_position.0 + x) * zoom_ratio - x,
                        (camera_position.1 + y) * zoom_ratio - y,
                    );
                }
            }
        }
        // pan while right mouse button is held
        if window.get_mouse_down(minifb::MouseButton::Right) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Pass) {
                if is_mouse_right_down {
                    camera_position = (mouse_pan_start.0 - x, mouse_pan_start.1 - y)
                } else {
                    mouse_pan_start = (camera_position.0 + x, camera_position.1 + y);
                    is_mouse_right_down = true;
                }
            }
        } else {
            is_mouse_right_down = false;
        }
        // create new molds when G key is pressed
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            for _ in 0..300 {
                let x = fastrand::usize(..GRID_X);
                let y = fastrand::usize(..GRID_Y);
                simulation.generate_mold(x, y);
            }
        }
        // delete everything when D key is pressed
        if window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
            simulation.clear();
        }
        // start/pause when P key is pressed
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            is_running = !is_running;
        }
        // decrease/increase light level when Q/W is pressed
        if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
            simulation.energy_light = 0.max(simulation.energy_light - 1)
        }
        if window.is_key_pressed(Key::W, minifb::KeyRepeat::No) {
            simulation.energy_light = 20.min(simulation.energy_light + 1)
        }

        // update simulation state
        if is_running {
            for _ in 0..1 {
                simulation.update();
            }
        }

        // update fps in window title
        let elapsed: Duration = last_frame_time.elapsed();
        last_frame_time = Instant::now();
        average_fps = 0.95 * average_fps + 0.05 / elapsed.as_secs_f64();

        window.set_title(
            format!(
                "rusty-mold - fps: {average_fps:.0} - light level: {0}",
                simulation.energy_light
            )
            .as_str(),
        );

        // render new state
        let camera_offset = (
            (camera_position.0).rem_euclid((GRID_X * zoom) as f32) as usize,
            (camera_position.1).rem_euclid((GRID_Y * zoom) as f32) as usize,
        );
        simulation.render(&mut buffer, buffer_size, camera_offset, zoom);
        window
            .update_with_buffer(&buffer, buffer_size.0, buffer_size.1)
            .unwrap();
    }
}
