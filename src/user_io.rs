use std::io::{self, Write};

// TO-DO: Rename and move this function
// TO-DO: Make it so it .trim()'s automatically
pub fn same_line_input(message: &str) -> String {
    // Temporary variable to store input as a string
    let mut input = String::new();

    // Print the message
    print!("{}", message);

    // Flush out return lines to get input on same line
    io::stdout().flush().unwrap();

    // Read the current line, throw error if we can't read it for some reason
    io::stdin().read_line(&mut input).expect("Could not read line!");

    // Returns string, trim any \r\n at the end
    input
}

// TO-DO: Understand what .unwrap() is doing
pub fn get_num_seconds() -> u64 {
    // Store input as a string
    let mut input_seconds = String::new();
    
    // Print number of seconds clock should run for on same line
    print!("⏰ Number of seconds the clock should run for: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_seconds).expect("Could not read line!");

    // Convert number to seconds, return it
    input_seconds.trim().parse::<u64>().unwrap()
}

// Clears all text from the console
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}