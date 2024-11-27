use std::path::Path;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
use std::env;
use std::collections::HashMap;
use log::{info, error, warn};

use rumqttc::{MqttOptions, Client, QoS};
use super::animations;
use super::config::Config;

type AnimationFactory = Arc<dyn Fn() -> Box<dyn animations::Animation>>;

pub struct App {
    config: Config,
    animation_factories: HashMap<String, AnimationFactory>,
    current_animation: Box<dyn animations::Animation>,
    next_animation_name: Arc<Mutex<String>>
}

impl App {
    pub fn new() -> App {
        let config = Config::default();

        let wheel_length = config.get_wheel_length();
        let strip_length = config.get_strip_length();

        // Build animation factories
        let mut animation_factories: HashMap<String, AnimationFactory> = HashMap::new();
        animation_factories.insert("rainbow".to_string(), Arc::new(move || Box::new(animations::Rainbow::new(strip_length, wheel_length))));
        animation_factories.insert("srainbow".to_string(), Arc::new(move || Box::new(animations::SRainbow::new(strip_length, wheel_length))));
        animation_factories.insert("off".to_string(), Arc::new(move || Box::new(animations::Off::new())));
        animation_factories.insert("chase".to_string(), Arc::new(move || Box::new(animations::Chase::new(wheel_length))));

        let off_animation = animation_factories.get("off").unwrap()();

        App {
            config,
            animation_factories,
            current_animation: off_animation,
            next_animation_name: Arc::new(Mutex::new("".to_string())),
        }
    }

    #[allow(dead_code)]
    pub fn from_file<T>(config_path: T) -> App
    where
        T: AsRef<Path>,
    {
        let config = Config::from_file(config_path);

        let wheel_length = config.get_wheel_length();
        let strip_length = config.get_strip_length();

        // Build animation factories
        let mut animation_factories: HashMap<String, AnimationFactory> = HashMap::new();
        animation_factories.insert("rainbow".to_string(), Arc::new(move || Box::new(animations::Rainbow::new(strip_length, wheel_length))));
        animation_factories.insert("srainbow".to_string(), Arc::new(move || Box::new(animations::SRainbow::new(strip_length, wheel_length))));
        animation_factories.insert("off".to_string(), Arc::new(move || Box::new(animations::Off::new())));
        animation_factories.insert("chase".to_string(), Arc::new(move || Box::new(animations::Chase::new(wheel_length))));

        let off_animation = animation_factories.get("off").unwrap()();

        App {
            config,
            animation_factories,
            current_animation: off_animation,
            next_animation_name: Arc::new(Mutex::new("".to_string())),
        }
    }

    pub fn start_mqtt_listener(&self) {
        let next_animation_name = Arc::clone(&self.next_animation_name);

        thread::spawn(move || {
            // MQTT
            let device_name = env::var("DEVICE_NAME").unwrap_or("DDD_leds".to_string());
            let mqtt_host = env::var("MQTT_HOST").unwrap_or("localhost".to_string());
            let mqtt_port = env::var("MQTT_PORT").unwrap_or("1883".to_string()).parse::<u16>().unwrap_or(1883);
            let mqtt_channel = env::var("MQTT_CHANNEL").unwrap_or("home/leds".to_string());

            let mut mqttoptions = MqttOptions::new(device_name, mqtt_host, mqtt_port);
            mqttoptions.set_keep_alive(time::Duration::new(60, 0));

            let (mut client, mut connection) = Client::new(mqttoptions, 10);
            client.subscribe(&mqtt_channel, QoS::AtLeastOnce).unwrap();

            for notification in connection.iter() {
                if let Ok(event) = notification {
                    if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = event {
                        if let Ok(s) = std::str::from_utf8(&p.payload) {
                            let mut next_animation_name = match next_animation_name.lock() {
                                Ok(n) => n,
                                Err(e) => {
                                    error!("Unable to lock next_animation_name: {}", e);
                                    continue;
                                }
                            };
                            *next_animation_name = s.to_string();
                        }
                    }
                } else if let Err(error) = notification {
                    warn!("Connection error {}\nTrying to reconnect...", error.to_string());
                    client.subscribe(&mqtt_channel, QoS::AtLeastOnce).unwrap();
                    continue;
                }
            }
        });
    }

    pub fn run(&mut self) {
        let mut controller: rs_ws281x::Controller = match ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                0,
                ChannelBuilder::new()
                    .pin(18)
                    .count(self.config.get_strip_length())
                    .strip_type(StripType::Ws2811Rgb)
                    .brightness(127)
                    .build()
            )
            .build() {
            Ok(c) => c,
            Err(e) => {
                error!("Unable to setup led controller: {}", e);
                return;
            }
        };

        loop {
            // Check if current animation is different to the next animation and that the current animation is not stopping
            if self.current_animation.name().ne(self.next_animation_name.lock().unwrap().as_str()) && !self.current_animation.stopping() {
                info!("Stopping animation: {}", self.current_animation.name());
                self.current_animation.stop();
            }

            // Save the result of next_frame to a variable so that we can check if the animation has changed
            let res: bool = self.current_animation.next_frame(&mut controller);

            // If the animation stopped, we can use next_animation to start the next animation
            if !res {
                let next_animation = match self.next_animation_name.lock() {
                    Ok(n) => n,
                    Err(e) => {
                        warn!("Unable to lock next_animation_name: {}", e);
                        continue;
                    }
                };

                // If the next animation is not empty, we can start it
                if !next_animation.is_empty() {
                    info!("Starting animation: {}", next_animation);

                    // Get the animation factory from the hashmap
                    let animation_factory = match self.animation_factories.get(next_animation.as_str()) {
                        Some(f) => f,
                        None => {
                            warn!("Unable to find animation factory for animation: `{}` defaulting to off", next_animation);
                            // Default to off and set string to off

                            let mut next_animation_name = match self.next_animation_name.lock() {
                                Ok(n) => n,
                                Err(e) => {
                                    error!("Unable to lock next_animation_name: {}", e);
                                    continue;
                                }
                            };
                            *next_animation_name = "off".to_string();

                            self.animation_factories.get("off").unwrap()
                        }
                    };

                    // Create the new animation
                    self.current_animation = animation_factory();
                    self.current_animation.start();
                }
            }

            controller.render().unwrap();
            thread::sleep(time::Duration::from_millis(self.current_animation.wait_time()));
        }
    }
}
