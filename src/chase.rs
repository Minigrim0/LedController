use rand::Rng;
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
    color: [u8; 4],
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
            color: [0, 0, 0, 0],
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
                false
            },
            STATUS::BUILDUP => {
                self.running = true;
                let leds = controller.leds_mut(0);
                if self.current_index < self.strip_length && leds[self.current_index as usize] == [0, 0, 0, 0] {
                    leds[self.current_index as usize] = self.color;
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

                true
            },
            STATUS::FADEOUT => {
                let leds = controller.leds_mut(0);
                if self.current_index >= 0 {
                    // Light up previous led & light off the current index's one
                    leds[self.current_index as usize] = [0, 0, 0, 0];
                    if self.current_index > 0 {
                        leds[(self.current_index - 1) as usize] = self.color;
                    }
                    self.current_index -= 1;
                    true
                } else {
                    // find first leds that is lit up
                    let leds = controller.leds_mut(0);
                    for index in 0..self.strip_length {
                        if leds[index as usize] != [0, 0, 0, 0] {
                            self.current_index = index;
                            break;
                        }
                    }
                    if self.current_index < 0 {
                        if self.running {
                            let mut rng = rand::thread_rng();

                            self.color = [
                                rng.gen(),
                                rng.gen(),
                                rng.gen(),
                                0
                            ];
                        }

                        self.current_index = 0;
                        self.status = STATUS::BUILDUP;
                    }
                    self.running
                }
            },
        }
    }

    fn start(&mut self) -> () {
        let mut rng = rand::thread_rng();

        self.running = true;
        self.status = STATUS::BUILDUP;
        self.color = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            0
        ];
    }

    fn stop(&mut self) -> () {
        self.running = false;
        self.status = STATUS::FADEOUT;
    }

    fn stopping(&self) -> bool {
        !self.running
    }

    fn name(&self) -> &str {
        "chase"
    }

    fn wait_time(&self) -> u64 {
        10
    }
}