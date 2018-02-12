extern crate communicator;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// convenient but could lead to naming collisions
use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;

    communicator::client::connect();
    communicator::network::connect();
    communicator::network::server::connect();
}