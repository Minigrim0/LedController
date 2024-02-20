use rs_ws281x::Controller;

use crate::animation::Animation;

enum STATUS {
    OFF,
    BUILDUP,
    FADEOUT
}

pub struct Chase {
    status: STATUS,  // 0 off, 1 build up, 2 fade out
    current_index: i32,
    strip_length: i32,
    running: bool,  // Becomes false when the animation should stop
}

// This animation occurs in two phases:
// 1. Build up: The first led is lit up, then the second, then the third, etc.
// 2. Fade out: The first led is turned off, then the second, then the third, etc.
impl Chase {
    pub fn new(strip_length: i32) -> Chase {
        Chase {
            status: STATUS::OFF,
            current_index: 0,
            strip_length,
            running: false,
        }
    }
}

impl Animation for Chase {
    fn next_frame(&mut self, controller: &mut Controller) -> bool {
        match self.status {
            STATUS::OFF => {
                self.running = false;
                return false;
            },
            STATUS::BUILDUP => {
                self.running = true;
                let leds = controller.leds_mut(0);
                if self.current_index < self.strip_length && leds[self.current_index as usize] == [0, 0, 0, 0] {
                    leds[self.current_index as usize] = [127, 127, 127, 0];
                    if self.current_index > 0 {
                        leds[(self.current_index - 1) as usize] = [0, 0, 0, 0];
                    }
                    self.current_index += 1;
                } else {
                    // Step into fade out phase
                    if self.current_index == 0 {
                        self.status = STATUS::FADEOUT;
                        self.current_index = -1;
                    }
                    self.current_index = 0;
                }

                return true;
            },
            STATUS::FADEOUT => {
                self.running = true;
                let leds = controller.leds_mut(0);
                if self.current_index >= 0 {
                    // Light up previous led & light off the current index's one
                    println!("FADEOUT - Turning off {}", self.current_index);
                    leds[self.current_index as usize] = [0, 0, 0, 0];
                    if self.current_index > 0 {
                        println!("        - Turning on  {}", self.current_index - 1);
                        leds[(self.current_index - 1) as usize] = [127, 127, 127, 0];
                    }
                } else {
                    // find first leds that is lit up
                    let leds = controller.leds_mut(0);
                    for index in 0..self.strip_length {
                        if leds[index as usize] != [0, 0, 0, 0] {
                            self.current_index = index;
                            break;
                        }
                    }
                }

                return true;
            },
        }
    }

    fn start(&mut self) -> () {
        self.running = true;
        self.status = STATUS::BUILDUP;
    }

    fn stop(&mut self) -> () {
        self.running = false;
    }

    fn stopping(&self) -> bool {
        !self.running
    }

    fn name(&self) -> &str {
        "chase"
    }
}