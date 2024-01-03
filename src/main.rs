use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
use rumqttc::{MqttOptions, Client, QoS};
use std::{thread, time};
use std::sync::{Arc, Mutex};

use crate::animation::Animation;

pub mod color;
pub mod animation;
pub mod rainbow;

fn main(){
    const WHEEL_LENGTH: i32 = 78;
    const STRIP_LENGTH: i32 = 96;

    let mut mqttoptions = MqttOptions::new("rust_client", "trappe.local", 1883);
    mqttoptions.set_keep_alive(time::Duration::new(5, 0));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("home/leds", QoS::AtLeastOnce).unwrap();

    let current_animation: Arc<Mutex<rainbow::Rainbow>> = Arc::new(Mutex::new(rainbow::Rainbow::new(STRIP_LENGTH, WHEEL_LENGTH)));
    let current_animation_clone = Arc::clone(&current_animation);

    thread::spawn(move || {
        for notification in connection.iter() {
            if let Ok(event) = notification {
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = event {
                    if let Ok(s) = std::str::from_utf8(&p.payload) {
                        println!("Received: {}", s);
                        let mut current_animation = current_animation_clone.lock().unwrap();
                        match s {
                            "rainbow" => {
                                *current_animation = rainbow::Rainbow::new(STRIP_LENGTH, WHEEL_LENGTH);
                                current_animation.start();
                            },
                            "off" => {
                                current_animation.stop();
                            },
                            _ => {
                                println!("Unknown command: {}", s);
                            }
                        }
                    }
                }
            }
        }
    });

    let mut controller: rs_ws281x::Controller = match ControllerBuilder::new()
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
                .brightness(127)
                .build()
        )
        .build() {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to setup led controller: {}", e);
            return;
        }
    };

    loop {
        let mut current_animation = current_animation.lock().unwrap();
        current_animation.next_frame(&mut controller);

        controller.render().unwrap();
        thread::sleep(time::Duration::from_millis(20));
    }
}
