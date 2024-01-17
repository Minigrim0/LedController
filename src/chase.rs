use rs_ws281x::Controller;

use crate::animation::Animation;

pub struct chase {
    status: i8,  // 0 off, 1 build up, 2 fade out
    current_index: i32,
    strip_length: i32,
    running: bool,  // Becomes false when the animation should stop
}

// This animation occurs in two phases:
// 1. Build up: The first led is lit up, then the second, then the third, etc.
// 2. Fade out: The first led is turned off, then the second, then the third, etc.
impl chase {
    pub fn new(strip_length: i32) -> chase {
        chase {
            status: 0,
            current_index: 0,
            strip_length,
            running: false,
        }
    }
}

impl Animation for Chase {
    fn next_frame(&mut self, controller: &mut Controller) -> bool {
        match status {
            0 => {
                self.running = false;
                return false;
            },
            1 => {
                self.running = true;
                let leds = controller.leds_mut(0);
                if leds[self.current_index as usize] == [0, 0, 0, 0] && self.current_index < self.strip_length {
                    leds[self.current_index as usize] = [127, 127, 127, 0];
                    leds[self.current_index - 1 as usize] = [0, 0, 0, 0];
                    self.current_index += 1;
                } else {
                    // Step into fade out phase
                    if self.current_index == 0 {
                        self.status = 2;
                    }
                    self.current_index = 0;
                }

                return true;
            },
            2 => {
                // find first leds that is lit up
                let leds = controller.leds_mut(0);
                let mut first_lit_led = 0;
                for index in 0..self.strip_length {
                    if leds[index as usize] != [0, 0, 0, 0] {
                        first_lit_led = index;
                        break;
                    }
                }

            },
        }
    }