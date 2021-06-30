use std::{io::{stdout, Write}, time::Duration};

use crossterm::{ExecutableCommand, 
  Result,
   event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers}, 
   execute, 
   style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}

};

const HELP: Event = Event::Key(KeyEvent {
  code: KeyCode::Char('h'),
  modifiers: KeyModifiers::CONTROL,
});

const PAUSE: Event = Event::Key(KeyEvent {
  code: KeyCode::Char('o'),
  modifiers: KeyModifiers::CONTROL,
});

const TIMER: Event = Event::Key(KeyEvent {
  code: KeyCode::Char('t'),
  modifiers: KeyModifiers::CONTROL,
});

const RESET: Event = Event::Key(KeyEvent {
  code: KeyCode::Char('r'),
  modifiers: KeyModifiers::CONTROL,
});

fn print_events() -> Result<()> {
  loop {
    if poll(Duration::from_millis(1_000))? {
      let event = read()?;

      println!("Event::{:?}\r", event);
      match_event(event);

      if event == Event::Key(KeyCode::Esc.into()) {
        break;
      }
    }
  }

  Ok(())
}

fn main() {
  greet_new_user();
  print_events();


  // let x = KeyEvent::from(KeyCode::Char('c'));
  // let help = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
  // let pause = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
  // let timer = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
  // let reset = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

  // let mut sync_stdin = input.read_sync();

  // loop {
  //   if let Some(event) = sync_stdin.next() {
  //     match event {
  //       KeyEvent::Char('h') => println!("Help"),
  //       InputEvent::Keyboard(KeyEvent::Ctrl('p')) => println!("Pause"),
  //       InputEvent::Keyboard(KeyEvent::Ctrl('t')) => println!("Timer"),
  //       InputEvent::Keyboard(KeyEvent::Ctrl('r')) => println!("Reset"),
  //     }
  //   }
  // }
}

fn match_event(event: Event) {
  match event {
      HELP => println!("Help"),
      PAUSE => println!("Pause"),
      TIMER => println!("Timer"),
      RESET => println!("Reset"),
      _ => {},
  }
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