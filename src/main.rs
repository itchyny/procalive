use std::env;

extern crate procalive;
use procalive::run::*;

fn main() {
    println!("{:?}", run(env::args().skip(1)));
}
