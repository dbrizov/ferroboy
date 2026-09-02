pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub struct Emulator {
    framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    frames: u64,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            frames: 0,
        }
    }

    pub fn run_frame(&mut self) {
        self.frames += 1;
        let shade = (self.frames / 30 % 4) as u8;
        self.framebuffer = [shade; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn framebuffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.framebuffer
    }
}
