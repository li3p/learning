// use tracing::info;
// use tracing_subscriber::prelude::*;

// mod custom_layer;
// use custom_layer::CustomLayer;
#![allow(unused)]
fn main() {
    // tracing_subscriber::registry().with(CustomLayer).init();
    // info!(a_bool = true, answer = 42, message = "Hello, world!");
    // let answer = "42";
    // let no_answer = answer;

    let answer = "42".to_string();
    let no_answer = answer.clone();

    println!("answer = {}", answer);
    println!("no_answer = {}", no_answer);
}
