use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use clap::{Parser};

use game_loop::{game_loop, Time, TimeTrait};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use chip8_engine::{Machine, Quirks};

const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / 60 as u64);

struct Emulator {
    machine: Machine,
    pixels: Pixels,
    input: WinitInputHelper,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to a file containing CHIP-8 bytecode
    #[clap(value_parser)]
    path: String,

    /// Whether quirks mode should be active (required for some games to work)
    #[clap(short, long, action)]
    quirks: bool,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let program_bytes = fs::read(cli.path).expect("Unable to find input file");
    let machine = Machine::new(program_bytes, Quirks::from_flag(cli.quirks));

    let event_loop = EventLoop::new();
    let window = get_window(&event_loop);
    let pixels = get_pixels(&window);

    let keyboard_mappings = get_keyboard_mappings();
    let input = WinitInputHelper::new();

    let emulator = Emulator { machine, pixels, input };

    game_loop(
        event_loop,
        window,
        emulator,
        480,
        0.1,
        move |g| {
            let keys_pressed = get_keys_pressed(&g.game.input, &keyboard_mappings);
            g.game.machine.tick(keys_pressed);
        },
        move |g| {
            let title = format!("UPS {}, FPS {}", g.updates_per_second, (1f64 / g.last_frame_time()) as u8);
            g.window.set_title(&title);
            g.game.machine.draw(g.game.pixels.get_frame());
            if let Err(e) = g.game.pixels.render() {
                error!("pixels.render() failed: {:?}", e);
                g.exit();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174
            let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if dt > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(dt));
            }
        },
        |g, event| {
            let input = &mut g.game.input;
            if input.update(event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    g.exit();
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height);
                }
            }
        }
    );
}

fn get_window(event_loop: &EventLoop<()>) -> Window {
    let output_size = LogicalSize::new(64 as f64, 32 as f64);
    let window_size = LogicalSize::new(640 as f64, 320 as f64);
    WindowBuilder::new()
        .with_min_inner_size(output_size)
        .with_inner_size(window_size)
        .build(event_loop)
        .unwrap()
}

fn get_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    Pixels::new(64 as u32, 32 as u32, surface_texture).unwrap()
}

fn get_keyboard_mappings() -> HashMap<VirtualKeyCode, u8> {
    HashMap::from([
        (VirtualKeyCode::X,     0x0),
        (VirtualKeyCode::Key1,  0x1),
        (VirtualKeyCode::Key2,  0x2),
        (VirtualKeyCode::Key3,  0x3),
        (VirtualKeyCode::Q,     0x4),
        (VirtualKeyCode::W,     0x5),
        (VirtualKeyCode::E,     0x6),
        (VirtualKeyCode::A,     0x7),
        (VirtualKeyCode::S,     0x8),
        (VirtualKeyCode::D,     0x9),
        (VirtualKeyCode::Z,     0xA),
        (VirtualKeyCode::C,     0xB),
        (VirtualKeyCode::Key4,  0xC),
        (VirtualKeyCode::R,     0xD),
        (VirtualKeyCode::F,     0xE),
        (VirtualKeyCode::V,     0xF)
    ])
}

fn get_keys_pressed(input: &WinitInputHelper, mappings: &HashMap<VirtualKeyCode, u8>) -> Vec<u8> {
    mappings
        .into_iter()
        .filter(|m| input.key_held(*m.0))
        .map(|m| *m.1)
        .collect()
}