use colog;

mod animations;
mod config;
mod app;
pub mod utils;

use app::App;

fn main(){
    colog::init();

    let mut app = App::new();
    app.start_mqtt_listener();
    app.run();
}
