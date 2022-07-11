use crossterm::event::{read, KeyEvent};

use crate::*;

// Waits for one keypress and return it.
pub fn editor_read_key() -> Result<KeyEvent> {
    loop{
        if let Ok(event) = read() {
            if let Key(key_event) = event{
                return Ok(key_event);
            }
        } else {
            die("Read error");
            break;
        }
    }
    unreachable!();
}