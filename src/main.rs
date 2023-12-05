use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};

fn hue_to_rgb(h: f64, s: f64, l: f64) -> (i32, i32, i32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as i32,
        ((g + m) * 255.0) as i32,
        ((b + m) * 255.0) as i32,
    )
}

fn main(){

    const WHEEL_LENGTH: i32 = 78;
    const STRIP_LENGTH: i32 = 96;

    let mut controller = match ControllerBuilder::new()
        // default
        .freq(800_000)
        // default
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(STRIP_LENGTH)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build()
        )
        .build() {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to setup led controller: {}", e);
            return;
        }
    };

    let mut angle: i32 = 0;

    loop {
        angle = (angle + 1) % 360;

        {
            let leds = controller.leds_mut(0);
            let mut last_led = [0, 0, 0, 0];
            for led in leds {
                let current_led = *led;
                *led = last_led;
                last_led = current_led;
            }
        }

        controller.render().unwrap();
    }
}
