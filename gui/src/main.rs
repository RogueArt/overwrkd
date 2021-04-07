use iced::{button, window, Align, Button, Slider, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    let settings: iced::Settings<()> = Settings {
            window: window::Settings {
                size: (10,20),
                resizable: true,
                decorations: true,
                min_size: None,
                max_size: None,
                transparent: false,
                always_on_top: true,
                icon: None,
            },
            ..Default::default()
        };

    Timer::run(settings)
}

#[derive(Default)]
// Manage program state
struct Timer {
    temp: i32,
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    // DecrementPressed,
    TextPressed,
}

impl Sandbox for Timer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Pomodoro Timer")
    }

    fn update(&mut self, message: Message) {
        println!("{:?}", message);
        // match message {
        //     Message::IncrementPressed => self.current_time += 1,
        //     Message::DecrementPressed => {} // self.current_time -= 1;,
        // }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("This is some sample text")
                    .size(50))
            .push(Button::new(&mut self.button_state, Text::new("Click me!")).on_press(Message::TextPressed))
            .into()
        
    }
}