
#![allow(dead_code)]

extern crate playlib;

use playlib::messages;
use playlib::slicers;
use playlib::hashers;
// use playlib::traffic::TrafficLight;
// use playlib::traffic::TrafficLight::{Red};
use playlib::first::top;
use playlib::types;
use playlib::rex;

fn run_tests() {
    playlib::test_vec();
    messages::hello();
    messages::bye();
    hashers::test1();
    slicers::test1();
    slicers::test_longest_string();
    top::test();
    println!("{:?}", types::Colors::White);
}

fn main() {
    rex::test1("2020-10-05");
    rex::test1("2020-1-2");
}
