use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::decode;

const SCREEN_WIDTH: u32 = 2500;
const SCREEN_HEIGHT: u32 = 600;

pub fn show_wav(wave_form: &decode::WaveForm) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window("waveform", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(pixels::Color::RGB(30, 30, 30));
    canvas.clear();
    let default_color = pixels::Color::RGB(0xCF, 0xCF, 0xCF);
    let second_color = pixels::Color::RGB(0xF0, 0x10, 0x10);
    canvas.set_draw_color(default_color);
    let max_sample = wave_form.wave_data.data.iter().max().unwrap();
    let min_sample = wave_form.wave_data.data.iter().min().unwrap();
    let sample_per_second: usize = wave_form.fmt_ck.sample_rate as usize;
    let step_size: usize = (wave_form.wave_data.size / SCREEN_WIDTH) as usize;

    println!("Data Value Range: {} - {}", max_sample, min_sample);
    for (i, sample) in wave_form
        .wave_data
        .data
        .iter()
        .enumerate()
        .step_by(step_size)
    {
        let norm_height =
            ((*sample as f64) / ((*max_sample - *min_sample) as f64)) * SCREEN_HEIGHT as f64 / 2.0;
        if i % sample_per_second < 100 && i > sample_per_second - 100 {
            println!("SETTING SECOND MARKER");
            canvas.set_draw_color(second_color);
            draw_line(i / step_size, SCREEN_HEIGHT as f64, &mut canvas);
        } else {
            canvas.set_draw_color(default_color);
            draw_line(i / step_size, norm_height, &mut canvas);
        }
    }
    canvas.present();
    let mut events = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }
                _ => {}
            }
        }
    }
}

fn draw_line(x: usize, y: f64, canvas: &mut Canvas<Window>) {
    let upper: i32 = (y as i32) + ((SCREEN_HEIGHT as i32) / 2);
    let lower: i32 = ((SCREEN_HEIGHT as i32) / 2) - (y as i32);
    canvas
        .draw_line(Point::new(x as i32, lower), Point::new(x as i32, upper))
        .unwrap();
}
