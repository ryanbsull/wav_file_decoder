use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::decode;

const SCREEN_WIDTH: u32 = 2500;
const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)]
struct ScreenSize {
    height: usize,
    width: usize,
}

struct SampleConfig {
    max: usize,
    min: usize,
    rate: usize,
}

#[allow(dead_code)]
struct WavWindowConfig {
    wave_color: Color,
    per_sec_color: Color,
    sample_config: SampleConfig,
    step_size: usize,
    size: ScreenSize,
}

impl WavWindowConfig {
    fn new(
        screen_height: usize,
        screen_width: usize,
        sample_max: usize,
        sample_min: usize,
        sample_rate: usize,
        sample_size: usize,
    ) -> Self {
        WavWindowConfig {
            wave_color: Color::RGB(0xCF, 0xCF, 0xCF),
            per_sec_color: Color::RGB(0xF0, 0x20, 0x20),
            sample_config: SampleConfig {
                max: sample_max,
                min: sample_min,
                rate: sample_rate,
            },
            step_size: sample_size / screen_width,
            size: ScreenSize {
                height: screen_height,
                width: screen_width,
            },
        }
    }
}

pub fn show_wav(wave_form: &decode::WaveForm) {
    let window_config: WavWindowConfig = WavWindowConfig::new(
        SCREEN_HEIGHT as usize,
        SCREEN_WIDTH as usize,
        *wave_form.wave_data.data.iter().max().unwrap() as usize,
        *wave_form.wave_data.data.iter().min().unwrap() as usize,
        wave_form.fmt_ck.sample_rate as usize,
        wave_form.wave_data.size as usize,
    );
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

    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    canvas.set_draw_color(window_config.wave_color);

    /*
        Go through the waveform data, incrementing by step_size and display
        to screen as a line extending up and down from the center of the
        screen
    */
    for (i, sample) in wave_form
        .wave_data
        .data
        .iter()
        .enumerate()
        .step_by(window_config.step_size)
    {
        // calculate the normalized height of the line
        let norm_height = ((*sample as f64)
            / ((window_config.sample_config.max - window_config.sample_config.min) as f64))
            * SCREEN_HEIGHT as f64
            / 2.0;
        // check if the samples are around the second mark
        if i % window_config.sample_config.rate < 100 && i > window_config.sample_config.rate - 100
        {
            // if around the second draw a red line to denote the second boundary
            canvas.set_draw_color(window_config.per_sec_color);
            draw_wav_line(
                i / window_config.step_size,
                SCREEN_HEIGHT as f64,
                &mut canvas,
            );
        } else {
            // otherwise just draw the waveform data as is
            canvas.set_draw_color(window_config.wave_color);
            draw_wav_line(i / window_config.step_size, norm_height, &mut canvas);
        }
    }
    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas
        .draw_line(
            Point::new(0, (SCREEN_HEIGHT as i32) / 2),
            Point::new(SCREEN_WIDTH as i32, (SCREEN_HEIGHT as i32) / 2),
        )
        .unwrap();
    canvas.present();

    // Init event loop to track key presses //
    let mut events = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in events.poll_iter() {
            match event {
                // allow the user to Ctrl-C out of the program
                Event::Quit { .. } => break 'main,
                // check if a key was pressed
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    // if the key is escape then exit
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }
                _ => {}
            }
        }
    }
}

fn draw_wav_line(x: usize, y: f64, canvas: &mut Canvas<Window>) {
    // calculate the lower and upper y values of the line since it extends both up and down from the center line
    let upper: i32 = (y as i32) + ((SCREEN_HEIGHT as i32) / 2);
    let lower: i32 = ((SCREEN_HEIGHT as i32) / 2) - (y as i32);
    canvas
        .draw_line(Point::new(x as i32, lower), Point::new(x as i32, upper))
        .unwrap();
}
