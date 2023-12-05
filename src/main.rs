use rs_ws281x::ControllerBuilder;

fn main(){

    let controller = ControllerBuilder::new()
        // default
        .freq(800_000)
        // default
        .dma(10)
        .channel(
            ChannelBuilder::new()
                .pin(18)
                .count(10)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build()
        )
        .build();

    // get the strand of LEDs on channel 1
    let leds = controller.leds_mut(0);
    // set the first LED to white (with the configured
    // strip above, this is BGRW)
    leds[0] = [255, 255, 255, 0];

    // render it to the strand
    controller.render();
}
