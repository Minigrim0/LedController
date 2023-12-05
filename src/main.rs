use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};

fn main(){

    let mut controller = match ControllerBuilder::new()
        // default
        .freq(800_000)
        // default
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(10)
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

    // get the strand of LEDs on channel 1
    let leds = controller.leds_mut(0);
    // set the first LED to white (with the configured
    // strip above, this is BGRW)
    leds[0] = [255, 255, 255, 0];

    // render it to the strand
    match controller.render() {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to render: {}", e);
            return;
        }
    }
}
