mod cube_net;
mod initialization;
mod interactive;
mod space;

use initialization::configuration::start;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    start(args);
}
