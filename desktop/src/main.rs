use std::sync::Arc;
use std::time::{Duration, Instant};

use ferroboy::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const GB_WIDTH: u32 = SCREEN_WIDTH as u32;
const GB_HEIGHT: u32 = SCREEN_HEIGHT as u32;
const WINDOW_SCALE: u32 = 4;
const FRAME_DURATION: Duration = Duration::from_nanos(16_742_706);

/// The DMG's four shades, lightest to darkest, as RGBA.
const PALETTE: [[u8; 4]; 4] = [
    [0x9B, 0xBC, 0x0F, 0xFF],
    [0x8B, 0xAC, 0x0F, 0xFF],
    [0x30, 0x62, 0x30, 0xFF],
    [0x0F, 0x38, 0x0F, 0xFF],
];

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    emulator: Emulator,
    next_frame: Instant,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            emulator: Emulator::new(),
            next_frame: Instant::now(),
        }
    }

    fn draw(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let framebuffer = self.emulator.framebuffer();
            let surface = pixels.frame_mut().as_chunks_mut::<4>().0;
            for (pixel, &shade) in surface.iter_mut().zip(framebuffer) {
                *pixel = PALETTE[shade as usize];
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("ferroboy")
            .with_inner_size(LogicalSize::new(
                GB_WIDTH * WINDOW_SCALE,
                GB_HEIGHT * WINDOW_SCALE,
            ));

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());

        self.pixels = Some(Pixels::new(GB_WIDTH, GB_HEIGHT, surface).unwrap());
        self.window = Some(window);
        self.next_frame = Instant::now() + FRAME_DURATION;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
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
            _ => return,
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_frame {
            self.next_frame += FRAME_DURATION;
            // After a stall (debugger, focus loss) run the next frame on schedule
            // instead of replaying the backlog at fast-forward speed.
            if self.next_frame < now {
                self.next_frame = now + FRAME_DURATION;
            }

            self.emulator.run_frame();
            self.draw();

            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
