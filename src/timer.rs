use std::{thread, time};
use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

// By default, loop waits for 1 second before performing actions
const WAIT_TIME: time::Duration = time::Duration::from_secs(1); 
// TO-DO: Should I go with u32 or u64 or u128?
#[derive(Debug)] // <-- TO-DO: Remove this
pub struct Timer {
    original_time: u64,
    current_time: u64,
}

impl Timer {
    // Create a new timer with a certain number of seconds
    pub fn new(seconds: u64) -> Self {
        Timer { original_time: seconds, current_time: seconds }
    }
    
    // Decrement the current time by one
    fn decrement(&mut self) -> &mut Self {
        self.current_time -= 1;
        self
    }
    
    // Reset current time to original time
    fn reset(&mut self) -> &mut Self {
        self.current_time = self.original_time;
        self
    }

    // REFACTOR: Should it by wait for a time or just wait()?
    fn wait(&mut self) -> &mut Self { 
        thread::sleep(WAIT_TIME); 
        self
    }

    fn notify(&self) { todo!(); }

    // Begin a timer 
    pub fn start(&mut self) {
        loop {
          
            if self.current_time == 0 {
                self.reset();
                // 
            }

            println!("Current time: {}", self.current_time);
            self.decrement();
            self.wait();
        }
    }
}
