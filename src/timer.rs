// GUI components
// use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

// Waiting functionality of timer
use std::{thread, time};

// User input/output
use std::io::{stdout, Write};
use crate::user_io;

const ONE_SECOND: time::Duration = time::Duration::from_secs(1);
// TO-DO: Should I go with u32 or u64 or u128?
// TO-DO: We can remove current_time completely
#[derive(Debug)] // <-- TO-DO: Remove this

pub struct Timer {
    state: TimerState,
    original_time: u64,
    current_time: u64,
}
// TO-DO: Add a state variable to keep track of state
#[derive(Debug)] // <-- TO-DO: Remove this
enum TimerState {
    Idle,
    Active,
    // PAUSED,
}

impl Timer {
    // Create a new timer with a certain number of seconds
    pub fn new(seconds: u64) -> Self {
        Timer {
            state: TimerState::Idle,
            original_time: seconds, 
            current_time: seconds 
        }
    }
    
    // Decrement the current time by one
    fn decrement(&mut self) -> &mut Self {
        self.current_time -= 1;
        self
    }
    
    // Reset current time to original time
    // Clear the console
    fn reset(&mut self) -> &mut Self {
        self.current_time = self.original_time;
        self
    }

    // REFACTOR: Should it by wait for a time or just wait()?
    fn wait(&mut self, wait_time: time::Duration) -> &mut Self { 
        thread::sleep(wait_time); 
        self
    }

    fn notify(&self) {
        // TO-DO: Add alert sound

        
    }

    // Format time as seconds as a String in mm:ss format
    fn format_time(&self, time: u64) -> String {
        let seconds = time % 60;
        let minutes = (time / 60) % 60;

        format!("{:0>2}:{:0>2}", minutes, seconds)
    }

    fn show_current_time(&mut self) -> &mut Self {
        print!("\r🕑 Current time: {} ", self.format_time(self.current_time));
        stdout().flush().unwrap();
        self
    }

    // TO-DO: Move this to outward facing set of code
    // TO-DO: Make a function for getting input
    fn keep_going(&self) -> bool {
        let input = user_io::same_line_input("🔔 Time is up! Would you like to restart the timer? (Y/N): ");
        input.trim().to_ascii_uppercase() == "Y" 
    }

    // Begin a timer 
    pub fn start(&mut self) {
        // Timer is no longer idle, now active
        self.state = TimerState::Active;

        // Count down starting from original time
        // Update current time each time for pausing 
        for _ in (0..self.original_time).rev() {
            self.show_current_time().decrement().wait(ONE_SECOND);
        }

        // Once timer is done, print a new line
        self.show_current_time();
        println!();

        // TO-DO: ALlow for timer to be paused, for it to restart
        match self.keep_going() {
            true => self.reset().start(),
            false => self.state = TimerState::Idle,
        }
    }
}
