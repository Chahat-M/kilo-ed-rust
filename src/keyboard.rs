use crossterm::event::{read, Event::*, KeyEvent};

use kilo_ed_rust::*;

pub struct Keyboard;

impl Keyboard {
    // Function that waits for one keypress and return it.
    pub fn read_key(&self) -> EditorResult<KeyEvent, ResultCode> {
        loop{
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                return Err(ResultCode::KeyReadFail);
            }
        }
    }
}
