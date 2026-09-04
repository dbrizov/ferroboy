use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use ferroboy::{Button, Emulator};
use ferroboy_common::{
    Audio, FRAME_DURATION, GB_HEIGHT, GB_WIDTH, WINDOW_SCALE, looks_like_a_rom, to_rgba,
};
use pixels::wgpu::TextureFormat;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::web::{EventLoopExtWebSys, WindowExtWebSys};
use winit::window::{Window, WindowId};

const SAVE_INTERVAL: Duration = Duration::from_secs(2);

fn button_for(code: &str) -> Option<Button> {
    match code {
        "ArrowRight" => Some(Button::Right),
        "ArrowLeft" => Some(Button::Left),
        "ArrowUp" => Some(Button::Up),
        "ArrowDown" => Some(Button::Down),
        "KeyX" => Some(Button::A),
        "KeyZ" => Some(Button::B),
        "Backspace" => Some(Button::Select),
        "Enter" => Some(Button::Start),
        _ => None,
    }
}

thread_local! {
    static PICKED_ROM: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
    static PRESSED_BUTTONS: RefCell<Vec<(Button, bool)>> = const { RefCell::new(Vec::new()) };
}

struct App {
    emulator: Emulator,
    save_key: Option<String>,
    audio: Rc<RefCell<Option<Audio>>>,
    window: Option<Arc<Window>>,
    pixels: Rc<RefCell<Option<Pixels<'static>>>>,
    next_frame: Instant,
    next_save: Instant,
}

const DEFAULT_ROM: &[u8] = include_bytes!("../roms/tobu.gb");

impl App {
    fn new(audio: Rc<RefCell<Option<Audio>>>) -> Self {
        let mut app = Self {
            emulator: Emulator::unplugged(),
            save_key: None,
            audio,
            window: None,
            pixels: Rc::new(RefCell::new(None)),
            next_frame: Instant::now(),
            next_save: Instant::now(),
        };
        app.load(DEFAULT_ROM);
        app
    }

    fn insert_rom(&mut self, rom: Vec<u8>) {
        if !looks_like_a_rom(&rom) {
            alert("That file does not look like a Game Boy ROM.");
            return;
        }

        self.write_save();
        self.load(&rom);
    }

    fn load(&mut self, rom: &[u8]) {
        self.emulator = Emulator::new(rom);
        self.save_key = Some(format!("ferroboy-sav-{:016x}", fnv1a(rom)));

        if let Some(saved) = self.read_save() {
            self.emulator.load_battery_ram(&saved);
        }
    }

    fn read_save(&self) -> Option<Vec<u8>> {
        let key = self.save_key.as_ref()?;
        let stored = local_storage()?.get_item(key).ok()??;
        from_hex(&stored)
    }

    fn write_save(&self) {
        let (Some(key), Some(ram)) = (&self.save_key, self.emulator.battery_ram()) else {
            return;
        };

        if let Some(storage) = local_storage() {
            let _ = storage.set_item(key, &to_hex(ram));
        }
    }

    fn play(&mut self) {
        let samples = self.emulator.take_samples();
        if let Some(audio) = self.audio.borrow().as_ref() {
            audio.queue(&samples);
        }
    }

    fn draw(&mut self) {
        let mut pixels = self.pixels.borrow_mut();
        let Some(pixels) = pixels.as_mut() else {
            return;
        };

        let surface = pixels.frame_mut().as_chunks_mut::<4>().0;
        to_rgba(&self.emulator, surface);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_inner_size(LogicalSize::new(
            GB_WIDTH * WINDOW_SCALE,
            GB_HEIGHT * WINDOW_SCALE,
        ));
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let canvas = window.canvas().unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("screen")
            .unwrap()
            .append_child(&canvas)
            .unwrap();

        let surface = SurfaceTexture::new(
            GB_WIDTH * WINDOW_SCALE,
            GB_HEIGHT * WINDOW_SCALE,
            window.clone(),
        );

        // Requesting a GPU adapter is asynchronous in the browser, so the
        // surface arrives a few frames after startup; draw() skips until then.
        // WebGPU canvases reject sRGB surface formats, and mixing sRGB and
        // linear between the texture and the surface would shift the colors.
        let cell = self.pixels.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let pixels = PixelsBuilder::new(GB_WIDTH, GB_HEIGHT, surface)
                .texture_format(TextureFormat::Rgba8Unorm)
                .surface_texture_format(TextureFormat::Bgra8Unorm)
                .build_async()
                .await
                .unwrap();
            *cell.borrow_mut() = Some(pixels);
        });

        self.window = Some(window);
        self.next_frame = Instant::now() + FRAME_DURATION;
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.borrow_mut().as_mut() {
                    let _ = pixels.resize_surface(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.borrow().as_ref() {
                    let _ = pixels.render();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(rom) = PICKED_ROM.with(|slot| slot.borrow_mut().take()) {
            self.insert_rom(rom);
        }
        PRESSED_BUTTONS.with(|queue| {
            for (button, pressed) in queue.borrow_mut().drain(..) {
                self.emulator.set_button(button, pressed);
            }
        });

        let now = Instant::now();
        if now >= self.next_frame {
            self.next_frame += FRAME_DURATION;
            // After a stall (hidden tab, focus loss) run the next frame on
            // schedule instead of replaying the backlog at fast-forward speed.
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

        if now >= self.next_save {
            self.next_save = now + SAVE_INTERVAL;
            self.write_save();
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

fn unlock_audio(audio: &Rc<RefCell<Option<Audio>>>) {
    let mut slot = audio.borrow_mut();
    if slot.is_none() {
        *slot = Audio::new();
    }
}

pub fn run() {
    console_error_panic_hook::set_once();

    let audio = Rc::new(RefCell::new(None));
    let document = web_sys::window().unwrap().document().unwrap();
    let key_audio = audio.clone();
    let on_key =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            unlock_audio(&key_audio);
            let Some(button) = button_for(&event.code()) else {
                return;
            };
            event.prevent_default();
            if event.repeat() {
                return;
            }
            let pressed = event.type_() == "keydown";
            PRESSED_BUTTONS.with(|queue| queue.borrow_mut().push((button, pressed)));
        });
    for kind in ["keydown", "keyup"] {
        document
            .add_event_listener_with_callback_and_bool(kind, on_key.as_ref().unchecked_ref(), true)
            .unwrap();
    }
    on_key.forget();

    let pointer_audio = audio.clone();
    let on_pointer = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        unlock_audio(&pointer_audio);
    });
    document
        .add_event_listener_with_callback_and_bool(
            "pointerdown",
            on_pointer.as_ref().unchecked_ref(),
            true,
        )
        .unwrap();
    on_pointer.forget();
    let input: web_sys::HtmlInputElement = document
        .get_element_by_id("rom")
        .unwrap()
        .dyn_into()
        .unwrap();

    let on_change = Closure::<dyn FnMut(web_sys::Event)>::new(|event: web_sys::Event| {
        let input: web_sys::HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let Some(file) = input.files().and_then(|files| files.get(0)) else {
            return;
        };

        wasm_bindgen_futures::spawn_local(async move {
            let Ok(buffer) = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await else {
                return;
            };
            let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
            PICKED_ROM.with(|slot| *slot.borrow_mut() = Some(bytes));
        });
    });
    input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    on_change.forget();

    let event_loop = EventLoop::new().unwrap();
    event_loop.spawn_app(App::new(audio));
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn alert(message: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.alert_with_message(message);
    }
}

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn to_hex(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn from_hex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }

    (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&text[index..index + 2], 16).ok())
        .collect()
}
