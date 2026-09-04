pub struct Length {
    maximum: u16,
    counter: u16,
    enabled: bool,
}

impl Length {
    pub fn new(maximum: u16) -> Self {
        Self {
            maximum,
            counter: 0,
            enabled: false,
        }
    }

    pub fn set(&mut self, value: u8) {
        self.counter = self.maximum - value as u16;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn counter(&self) -> u16 {
        self.counter
    }

    pub fn restore(&mut self, counter: u16) {
        self.counter = counter;
    }

    // Enabling the counter while the sequencer's next step is not a length step
    // steals one extra clock. Answers whether that emptied the counter with no
    // trigger in the same write to refill it, which silences the channel.
    pub fn write_control(&mut self, enabled: bool, trigger: bool, next_clocks: bool) -> bool {
        let steals_a_clock = !next_clocks && !self.enabled && enabled && self.counter > 0;
        self.enabled = enabled;
        if !steals_a_clock {
            return false;
        }

        self.counter -= 1;
        self.counter == 0 && !trigger
    }

    pub fn trigger(&mut self, next_clocks: bool) {
        if self.counter != 0 {
            return;
        }

        self.counter = self.maximum;
        if self.enabled && !next_clocks {
            self.counter -= 1;
        }
    }

    pub fn clock(&mut self) -> bool {
        if !self.enabled || self.counter == 0 {
            return false;
        }

        self.counter -= 1;
        self.counter == 0
    }
}
