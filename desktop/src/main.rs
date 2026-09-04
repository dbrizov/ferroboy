use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use ferroboy::{Button, Emulator};
use ferroboy_common::{Audio, FRAME_DURATION, GB_HEIGHT, GB_WIDTH, WINDOW_SCALE, to_rgba};
use gilrs::{EventType, Gilrs};
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const KEYBOARD: [(KeyCode, Button); 8] = [
    (KeyCode::ArrowRight, Button::Right),
    (KeyCode::ArrowLeft, Button::Left),
    (KeyCode::ArrowUp, Button::Up),
    (KeyCode::ArrowDown, Button::Down),
    (KeyCode::KeyX, Button::A),
    (KeyCode::KeyZ, Button::B),
    (KeyCode::Backspace, Button::Select),
    (KeyCode::Enter, Button::Start),
];

const PAD: [(gilrs::Button, Button); 8] = [
    (gilrs::Button::DPadRight, Button::Right),
    (gilrs::Button::DPadLeft, Button::Left),
    (gilrs::Button::DPadUp, Button::Up),
    (gilrs::Button::DPadDown, Button::Down),
    (gilrs::Button::South, Button::B),
    (gilrs::Button::East, Button::A),
    (gilrs::Button::Select, Button::Select),
    (gilrs::Button::Start, Button::Start),
];

struct App {
    save: Option<PathBuf>,
    audio: Option<Audio>,
    pad: Option<Gilrs>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    emulator: Emulator,
    next_frame: Instant,
}

impl App {
    fn new(emulator: Emulator, save: Option<PathBuf>) -> Self {
        let audio = Audio::new();
        if audio.is_none() {
            eprintln!("no audio output device; running silent");
        }

        Self {
            save,
            audio,
            pad: Gilrs::new().ok(),
            window: None,
            pixels: None,
            emulator,
            next_frame: Instant::now(),
        }
    }

    fn write_save(&self) {
        let (Some(path), Some(ram)) = (&self.save, self.emulator.battery_ram()) else {
            return;
        };

        if let Err(error) = std::fs::write(path, ram) {
            eprintln!("could not write {}: {error}", path.display());
        }
    }

    fn poll_pad(&mut self) {
        let Some(pad) = &mut self.pad else {
            return;
        };

        while let Some(event) = pad.next_event() {
            let (pad_button, pressed) = match event.event {
                EventType::ButtonPressed(button, _) => (button, true),
                EventType::ButtonReleased(button, _) => (button, false),
                _ => continue,
            };

            for (from, to) in PAD {
                if from == pad_button {
                    self.emulator.set_button(to, pressed);
                }
            }
        }
    }

    fn play(&mut self) {
        let samples = self.emulator.take_samples();
        if let Some(audio) = &self.audio {
            audio.queue(&samples);
        }
    }

    fn draw(&mut self) {
        let Some(pixels) = &mut self.pixels else {
            return;
        };

        let surface = pixels.frame_mut().as_chunks_mut::<4>().0;
        to_rgba(&self.emulator, surface);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("FerroBoy")
            .with_inner_size(LogicalSize::new(
                GB_WIDTH * WINDOW_SCALE,
                GB_HEIGHT * WINDOW_SCALE,
            ));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        center(&window);

        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());

        self.pixels = Some(Pixels::new(GB_WIDTH, GB_HEIGHT, surface).unwrap());
        self.window = Some(window);
        self.next_frame = Instant::now() + FRAME_DURATION;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.write_save();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &self.pixels {
                    let _ = pixels.render();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };
                let pressed = event.state == ElementState::Pressed;

                for (key, button) in KEYBOARD {
                    if key == code {
                        self.emulator.set_button(button, pressed);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.poll_pad();

        let now = Instant::now();
        if now >= self.next_frame {
            self.next_frame += FRAME_DURATION;
            // After a stall (debugger, focus loss) run the next frame on schedule
            // instead of replaying the backlog at fast-forward speed.
            if self.next_frame < now {
                self.next_frame = now + FRAME_DURATION;
            }

            self.emulator.run_frame();
            self.play();
            self.draw();

            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

fn center(window: &Window) {
    let Some(monitor) = window.current_monitor() else {
        return;
    };

    let screen = monitor.size();
    let outer = window.outer_size();
    let origin = monitor.position();

    window.set_outer_position(PhysicalPosition::new(
        origin.x + (screen.width as i32 - outer.width as i32) / 2,
        origin.y + (screen.height as i32 - outer.height as i32) / 2,
    ));
}

fn main() {
    let (emulator, save) = match std::env::args().nth(1) {
        Some(path) => {
            let rom = std::fs::read(&path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
            let mut emulator = Emulator::new(&rom);

            let save = PathBuf::from(&path).with_extension("sav");
            if let Ok(saved) = std::fs::read(&save) {
                emulator.load_battery_ram(&saved);
            }

            (emulator, Some(save))
        }
        None => (Emulator::unplugged(), None),
    };

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(emulator, save);
    event_loop.run_app(&mut app).unwrap();
}
