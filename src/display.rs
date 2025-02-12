use byteorder::{ByteOrder, LittleEndian};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::decode;

#[allow(dead_code)]
struct ScreenSize {
    height: usize,
    width: usize,
}

#[allow(dead_code)]
struct SampleConfig {
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
        sample_rate: usize,
        sample_size: usize,
    ) -> Self {
        WavWindowConfig {
            wave_color: Color::RGB(0xCF, 0xCF, 0xCF),
            per_sec_color: Color::RGB(0xF0, 0x20, 0x20),
            sample_config: SampleConfig { rate: sample_rate },
            step_size: sample_size / screen_width,
            size: ScreenSize {
                height: screen_height,
                width: screen_width,
            },
        }
    }
}

fn scale_data(
    wave_form: &decode::WaveForm,
    drawn_lines: &mut Vec<f64>,
    window_config: &WavWindowConfig,
) -> (f64, f64) {
    let step = wave_form.fmt_ck.bits_per_sample / 8;
    for i in (0..(wave_form.wave_data.data.len() - 1)).step_by(window_config.step_size) {
        // calculate the normalized height of the line
        let mut sample_8 = 0;
        let mut sample_16 = 0;
        if step == 1 {
            sample_8 = wave_form.wave_data.data[i];
        } else if step == 2 {
            sample_16 = LittleEndian::read_u16(&wave_form.wave_data.data[i..=(i + 1)]);
        }
        let line_height;
        if i == 0 {
            if step == 1 {
                line_height = sample_8 as f64
            } else {
                line_height = sample_16 as f64
            }
        } else {
            let mut avg_height: f64 = 0.0;
            for s in ((i - window_config.step_size + 1)..=i).step_by(step as usize) {
                if step == 1 {
                    avg_height += wave_form.wave_data.data[s] as f64;
                } else {
                    avg_height +=
                        LittleEndian::read_u16(&wave_form.wave_data.data[s..=(s + 1)]) as f64;
                }
            }
            avg_height = (step as f64) * avg_height / (window_config.step_size as f64);
            line_height = avg_height
        }
        if line_height == 0.0 {
            continue;
        }
        drawn_lines.push(line_height);
    }

    let max = drawn_lines
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let min = drawn_lines.iter().copied().fold(f64::INFINITY, f64::min);
    (max, min)
}

pub fn show_wav(wave_form: &decode::WaveForm) {
    let mode = core_graphics::display::CGDisplay::main()
        .display_mode()
        .unwrap();
    let mut drawn_lines: Vec<f64> = vec![];
    let height = mode.height() * 3 / 4; // scale screen to 3/4 of full screen height
    let width = mode.width();
    let window_config: WavWindowConfig = WavWindowConfig::new(
        height as usize,
        width as usize,
        wave_form.fmt_ck.sample_rate as usize,
        wave_form.wave_data.size as usize,
    );
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window("waveform", width as u32, height as u32)
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
    let line_max: f64;
    let line_min: f64;
    (line_max, line_min) = scale_data(&wave_form, &mut drawn_lines, &window_config);
    for (i, line) in drawn_lines.iter().enumerate() {
        let norm_height: f64 = ((*line - line_min) / (line_max - line_min)) * (height as f64 / 4.0);
        draw_wav_line(i, norm_height, height, &mut canvas);
    }
    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas
        .draw_line(
            Point::new(0, (height as i32) / 2),
            Point::new(width as i32, (height as i32) / 2),
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

fn draw_wav_line(x: usize, y: f64, height: u64, canvas: &mut Canvas<Window>) {
    // calculate the lower and upper y values of the line since it extends both up and down from the center line
    let upper: i32 = (y as i32) + ((height as i32) / 2);
    let lower: i32 = ((height as i32) / 2) - (y as i32);
    canvas
        .draw_line(Point::new(x as i32, lower), Point::new(x as i32, upper))
        .unwrap();
}

#[allow(dead_code)]
fn open_application(_wave_form: &decode::WaveForm, height: u64, width: u64) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window("waveform", width as u32, height as u32)
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
}
