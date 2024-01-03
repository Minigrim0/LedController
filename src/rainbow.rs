use rs_ws281x::Controller;

use crate::animation::Animation;
use crate::color::hue_to_rgb;

/// This struct represents a simple rainbow animation
pub struct Rainbow {
    angle: i32,
    strip_length: i32,
    wheel_length: i32,
    running: bool,  // Becomes false when the animation should stop
}

impl Rainbow {
    pub fn new(strip_length: i32, wheel_length: i32) -> Rainbow {
        Rainbow {
            angle: 0,
            strip_length,
            wheel_length,
            running: false,
        }
    }
}

impl Animation for Rainbow {
    /// Computes and renders the next frame of the animation to the controller
    /// 
    /// # Arguments
    /// 
    /// * `controller` - The controller to render the next frame to
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the animation is still running, false otherwise
    fn next_frame(&mut self, controller: &mut Controller) -> bool {
        self.angle = (self.angle + 1) % 360;

        {
            let leds = controller.leds_mut(0);
            let mut last_led = [0, 0, 0, 0];
            for index in 0..self.wheel_length {
                let current_led = leds[index as usize];
                leds[index as usize] = last_led;
                last_led = current_led;
            }
            for index in self.wheel_length..self.strip_length {
                if leds[index as usize][0] < 127 && self.running {
                    for elem in 0..2 {
                        leds[index as usize][elem] += 1;
                    }
                } else if leds[index as usize][0] > 0 && !self.running {
                    for elem in 0..2 {
                        leds[index as usize][elem] -= 1;
                    }
                } else {
                    leds[index as usize] = [127, 127, 127, 0];
                }
            }
        }
        if self.running {
            let leds = controller.leds_mut(0);
            let res = hue_to_rgb(self.angle as f64, 1.0, 0.5);
            leds[0] = [res.2, res.1, res.0, 0];
        } else {
            let leds = controller.leds_mut(0);
            leds[0] = [0, 0, 0, 0]; // Turn off the first led (will propagate to the rest of the strip)
        }

        self.running || controller.leds(0)[(self.wheel_length - 1) as usize] != [0, 0, 0, 0]
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
        "rainbow"
    }
}
