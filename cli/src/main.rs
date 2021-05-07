use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() {
  greet_new_user();
}

fn get_pause() {
  
}

fn greet_new_user() {
  execute!(
    stdout(),
    Print("🎉 Welcome to Pom!🎉\n"),
    Print("It looks like this is the first time you're using this...\n"),
    Print("Here's a quick-start:\n"),

    SetForegroundColor(Color::Blue),
    Print("📖 Ctrl + H -> Help\n"),
    Print("⏸  Ctrl + P -> Pause/Settings\n"),
    Print("⏲  Ctrl + T -> Timer\n"),
    Print("🔄 Ctrl + R -> Reset Timer\n"),
    ResetColor,

    Print("You can start the timer when you're ready!"),
    ResetColor,
  );
}