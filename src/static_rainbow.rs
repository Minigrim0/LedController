use rs_ws281x::Controller;

use crate::animation::Animation;
use crate::color::hue_to_rgb;

enum STATUS {
    FADEIN,
    ONGOING,
    FADEOUT
}

const MAX_BRIGHTNESS: f64 = 127.0;

/// This struct represents a simple srainbow animation
pub struct SRainbow {
    angle: i32,
    strip_length: i32,
    wheel_length: i32,
    status: STATUS,
    brightness: u8,
    running: bool,  // Becomes false when the animation should stop
}

impl SRainbow {
    pub fn new(strip_length: i32, wheel_length: i32) -> SRainbow {
        SRainbow {
            angle: 0,
            strip_length,
            wheel_length,
            status: STATUS::FADEIN,
            brightness: 0,
            running: false,
        }
    }
}

fn brightnessed(color: (u8, u8, u8), brightness: u8) -> [u8; 4] {
    [
        ((color.0 as f64) * (brightness as f64) / MAX_BRIGHTNESS) as u8,
        ((color.1 as f64) * (brightness as f64) / MAX_BRIGHTNESS) as u8,
        ((color.2 as f64) * (brightness as f64) / MAX_BRIGHTNESS) as u8,
        0
    ]
}

impl Animation for SRainbow {
    fn next_frame(&mut self, controller: &mut Controller) -> bool {
        self.angle = (self.angle + 1) % 360;

        match self.status {
            STATUS::FADEIN => {
                if self.brightness < 127 {
                    self.brightness += 1;
                } else {
                    self.status = STATUS::ONGOING;
                }
            },
            STATUS::FADEOUT => {
                if self.brightness > 0 {
                    self.brightness -= 1;
                } else {
                    self.running = false;
                }
            },
            _ => {}
        }

        let leds = controller.leds_mut(0);
        let res = hue_to_rgb(self.angle as f64, 1.0, 0.5);
        for x in 0..self.wheel_length {
            leds[x as usize] = brightnessed(res, self.brightness);
        }
        for x in self.wheel_length..self.strip_length {
            leds[x as usize] = brightnessed((127, 127, 127), self.brightness);
        }

        self.running
    }

    fn start(&mut self) -> () {
        self.running = true;
        self.brightness = 0;
        self.status = STATUS::FADEIN;
    }

    fn stop(&mut self) -> () {
        self.status = STATUS::FADEOUT;
    }

    fn stopping(&self) -> bool {
        matches!(self.status, STATUS::FADEOUT)
    }

    fn name(&self) -> &str {
        "srainbow"
    }

    fn wait_time(&self) -> u64 {
        20
    }
}
