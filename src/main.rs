use colog;
use clap::Parser;

mod animations;
mod config;
mod app;
mod args;
mod utils;

use app::App;
use config::Config;

fn main(){
    colog::init();

    let args = args::Args::parse();

    if args.dump_default_config {
        Config::default().dump();
    } else if args.dump_config {
        Config::from_file(&args.config_file).dump();
    } else {
        let mut app = App::new();
        app.start_mqtt_listener();
        app.run();
    }
}
