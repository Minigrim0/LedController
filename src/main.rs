use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
use rumqttc::{MqttOptions, Client, QoS};
use std::collections::HashMap;
use std::{thread, time};
use std::sync::{Arc, Mutex};

use crate::animation::Animation;

pub mod color;
pub mod animation;
pub mod rainbow;
pub mod off;
pub mod chase;

fn main(){
    const WHEEL_LENGTH: i32 = 78;
    const STRIP_LENGTH: i32 = 96;

    // Animation factory type
    type AnimationFactory = Arc<dyn Fn() -> Box<dyn Animation>>;

    let mut animation_factories: HashMap<String, AnimationFactory> = HashMap::new();
    animation_factories.insert("rainbow".to_string(), Arc::new(|| Box::new(rainbow::Rainbow::new(STRIP_LENGTH, WHEEL_LENGTH))));
    animation_factories.insert("off".to_string(), Arc::new(|| Box::new(off::Off::new())));
    animation_factories.insert("chase".to_string(), Arc::new(|| Box::new(chase::Chase::new(WHEEL_LENGTH))));

    // Animation holder
    let mut current_animation: Box<dyn Animation> = animation_factories.get("off").unwrap()();

    let next_animation_name: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
    let next_animation_name_clone = Arc::clone(&next_animation_name);

    thread::spawn(move || {
        // MQTT
        let mut mqttoptions = MqttOptions::new("3d_leds", "192.168.68.61", 1883);
        mqttoptions.set_keep_alive(time::Duration::new(60, 0));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        client.subscribe("home/leds", QoS::AtLeastOnce).unwrap();

        for notification in connection.iter() {
            if let Ok(event) = notification {
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = event {
                    if let Ok(s) = std::str::from_utf8(&p.payload) {
                        let mut next_animation_name = match next_animation_name_clone.lock() {
                            Ok(n) => n,
                            Err(e) => {
                                println!("Unable to lock next_animation_name: {}", e);
                                continue;
                            }
                        };
                        *next_animation_name = s.to_string();
                    }
                }
            } else if let Err(error) = notification {
                println!("Connection error {}\nTrying to reconnect...", error.to_string());
                client.subscribe("home/leds", QoS::AtLeastOnce).unwrap();
                continue;
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
        // Check if current animation is different to the next animation and that the current animation is not stopping
        if current_animation.name().ne(next_animation_name.lock().unwrap().as_str()) && !current_animation.stopping() {
            println!("Stopping animation: {}", current_animation.name());
            current_animation.stop();
        }

        // Save the result of next_frame to a variable so that we can check if the animation has changed
        let res: bool = current_animation.next_frame(&mut controller);

        // If the animation stopped, we can use next_animation to start the next animation
        if !res {
            let next_animation = match next_animation_name.lock() {
                Ok(n) => n,
                Err(e) => {
                    println!("Unable to lock next_animation_name: {}", e);
                    continue;
                }
            };

            // If the next animation is not empty, we can start it
            if !next_animation.is_empty() {
                println!("Starting animation: {}", next_animation);

                // Get the animation factory from the hashmap
                let animation_factory = match animation_factories.get(next_animation.as_str()) {
                    Some(f) => f,
                    None => {
                        println!("Unable to find animation factory for animation: `{}` defaulting to off", next_animation);
                        animation_factories.get("off").unwrap()
                    }
                };

                // Create the new animation
                current_animation = animation_factory();
                current_animation.start();
            }
        }

        controller.render().unwrap();
        thread::sleep(time::Duration::from_millis(current_animation.wait_time()));
    }
}
