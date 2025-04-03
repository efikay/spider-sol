use crossterm::event::{Event, KeyCode, read};

pub fn wait_for_enter_press(message: &String) {
    println!("{}", message);

    loop {
        if let Ok(Event::Key(event)) = read() {
            if event.code == KeyCode::Enter {
                break;
            }
        }
    }
}
