use rs_ws281x::Controller;
use crate::animation::Animation;

/// This structure represents the off animation
pub struct Off {
    running: bool,
}

impl Off {
    pub fn new() -> Off {
        Off {
            running: false,
        }
    }
}

impl Animation for Off {
    fn next_frame(&mut self, controller: &mut Controller) -> bool {
        if self.running {
            let leds = controller.leds_mut(0);
            leds[0] = [0, 0, 0, 0]; // Turn off the first led (will propagate to the rest of the strip)
        }

        self.running
    }

    fn start(&mut self) -> () {
        self.running = true;
    }

    fn stop(&mut self) -> () {
        self.running = false;
    }

    fn stopping(&self) -> bool {
        !self.running
    }

    fn name(&self) -> &str {
        "off"
    }
}