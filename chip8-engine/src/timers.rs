
pub struct Timers {
    pub delay: u8,
    pub sound: u8,
    time_until_tick: u8,
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            delay: 0,
            sound: 0,
            time_until_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.time_until_tick != 0 {
            self.time_until_tick -= 1;
            return;
        }

        self.time_until_tick = 8;

        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}