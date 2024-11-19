use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::decode;

pub fn show_wav(wave_form: &decode::WaveForm) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window("waveform", 2000, 600)
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

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    let color = pixels::Color::RGB(0xFF, 0xFF, 0xFF);
    canvas.set_draw_color(color);
    let max_sample = wave_form.wave_data.data.iter().max().unwrap();
    let min_sample = wave_form.wave_data.data.iter().min().unwrap();

    println!("Data Value Range: {} - {}", max_sample, min_sample);
    for (i, sample) in wave_form.wave_data.data.iter().enumerate() {
        let norm_height = ((*sample as f64) / ((*max_sample - *min_sample) as f64)) * 600 as f64;
        draw_point(i, norm_height, &mut canvas);
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

fn draw_point(x: usize, y: f64, canvas: &mut Canvas<Window>) {
    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
}
