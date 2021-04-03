use std::io::{self, Write};

pub mod timer;
use timer::Timer;
// Testsasdf asdf sdf 
// TO-DO: Understand what .unwrap() is doing
fn get_num_seconds() -> u64 {
    // Store input as a string
    let mut input_seconds = String::new();
    
    // Print number of seconds clock should run for on same line
    print!("Number of seconds the clock should run for: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_seconds).expect("Could not read line!");

    // Convert number to seconds, return it
    input_seconds.trim().parse::<u64>().unwrap()
}

fn main() {
    let seconds = get_num_seconds();
    let mut timer = Timer::new(seconds);
    timer.start();
}
