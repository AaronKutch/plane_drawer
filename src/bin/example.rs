use std::{
    thread,
    time::{Duration, Instant}
};
use minifb::{Key, Window, WindowOptions};
use plane_drawer::{Angle, Cam, CamData, Color, Pos, Vec2d, D1, D2, D3};

const DEFAULT_DIMENSIONS: (usize, usize) = (512, 512);
const FPS: usize = 50;

/// The inverse of the frequency of input handling
const INPUT_PERIOD_NANOS: u128 = 1_000_000_000 / FPS as u128;

fn main() {
    let mut winsize = (DEFAULT_DIMENSIONS.0, DEFAULT_DIMENSIONS.1);
    let mut cam_data = CamData::new(winsize, 1);
    let mut window_buf = Vec2d::new_with_fill(winsize, |_| 0x00_00_00_00);

    let mut window = Window::new(
        "Example",
        DEFAULT_DIMENSIONS.0,
        DEFAULT_DIMENSIONS.1,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();
    // we handle the update rate ourselves
    window.set_target_fps(FPS);
    window.set_background_color(50, 50, 50);
    window.set_cursor_visibility(true);

    let rerender = true;
    let mut time_fuel = 0u128;
    // call `Instant::now` as the last thing before the loop
    let mut old_time = Instant::now();



    loop {
        // if window changes size
        if window.get_size() != winsize {
            winsize = window.get_size();
            cam_data.update_winsize_and_buffsize(winsize);
            let new_winsize = cam_data.winsize();
            window_buf = Vec2d::new_with_fill(new_winsize, |_| 0x00_00_00_00);
        }


        // after setting background color, we can draw on the buffer
        window_buf.get_mut_flat1().fill(Color::new_from_u8((80, 50, 80)).to_u32());
        // --------------------------------------------------------------
        // draw relative to world origin
        if let Some(mut cam) = Cam::new_rel_to_pos(&mut window_buf, &cam_data, Pos::zero(), 255) {
            cam.draw_circle(D2::zero(), D1::from(5), Color::ultramarine_blue());
            cam.draw_circle(D2::from((10, 5)), D1::from(5), Color::ultramarine_blue());
            cam.draw_line((D2::zero(), D2::from((-10, 10))), Color::ultramarine_blue());
            cam.draw_rect_corners((D2::from((-5, -5)), D2::from((5, -10))), Color::ultramarine_blue());
            cam.draw_wide_pixel(D2::from((-5, -15)), 3, Color::ultramarine_blue());
        }

        // draw relative to (20, 20, 50) with a 1/4 pi angle
        if let Some(mut cam) = Cam::new_rel_to_pos(&mut window_buf, &cam_data, Pos::from((D3::from((20, 20, 50)), Angle::one_fourth_pi())), 255) {
            cam.draw_circle(D2::zero(), D1::from(5), Color::red());
            cam.draw_circle(D2::from((10, 5)), D1::from(5), Color::red());
            cam.draw_line((D2::zero(), D2::from((-10, 10))), Color::red());
            cam.draw_rect_corners((D2::from((-5, -5)), D2::from((5, -10))), Color::red());
            cam.draw_wide_pixel(D2::from((-5, -15)), 3, Color::red());

        }
        // --------------------------------------------------------------


        // just some minifb updating
        if rerender {
            let (width, height) = window_buf.len();
            window.update_with_buffer(window_buf.get_flat1(), width, height).unwrap();
        } else {
            // needed for `minifb` to handle input even if `update_with_buffer` is not called
            window.update();
        }
        if !window.is_open() || window.is_key_down(Key::Escape) {
            println!("Window closed, exiting program");
            return
        }


        // input handling
        // --------------------------------------------------------------
        window.get_keys_pressed(minifb::KeyRepeat::No).iter().for_each(|key| 
            match key {
                Key::O => {
                    println!("cam info: {:?}", cam_data);
                }
                Key::Z => {
                    cam_data.change_resolution(false, window.get_size());
                }
                Key::X => {
                    cam_data.change_resolution(true, window.get_size());
                }
                _ => {}
            }
        );
        window.get_keys().iter().for_each(|key| 
            match key {
                Key::W => {
                    cam_data.cam_pos_rel.coords.y += cam_data.cam_move_speed*cam_data.zoom();
                }
                Key::A => {
                    cam_data.cam_pos_rel.coords.x -= cam_data.cam_move_speed*cam_data.zoom();
                }
                Key::S => {
                    cam_data.cam_pos_rel.coords.y -= cam_data.cam_move_speed*cam_data.zoom();
                }
                Key::D => {
                    cam_data.cam_pos_rel.coords.x += cam_data.cam_move_speed*cam_data.zoom();
                }
                _ => {}
            }
        );
        window.get_scroll_wheel().map(|scroll|
            if scroll.1 > 0.0 {
                cam_data.change_zoom_exponent(true);
                cam_data.set_zoom(cam_data.target_zoom());
            } else if scroll.1 < 0.0 {
                cam_data.change_zoom_exponent(false);
                cam_data.set_zoom(cam_data.target_zoom());
            }
        );
        // --------------------------------------------------------------


        // efficiently wait without consuming a large amount of CPU time with a spin loop
        loop {
            let new_time = Instant::now();
            time_fuel += new_time.saturating_duration_since(old_time).as_nanos();
            old_time = new_time;
            if time_fuel < INPUT_PERIOD_NANOS {
                // wait for enough time fuel
                thread::sleep(Duration::from_nanos(
                    (INPUT_PERIOD_NANOS - time_fuel) as u64,
                ))
            } else {
                // consume and break for the next main loop iteration
                time_fuel -= INPUT_PERIOD_NANOS;
                break
            }
        }
        if time_fuel > (2 * INPUT_PERIOD_NANOS) {
            // it is taking longer than the input period for other parts of the main loop to
            // be processed, we are lagging
            time_fuel = INPUT_PERIOD_NANOS;
            // println!("lagging");
        }
    }
}