use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

use crate::keyboard::*;

use crate::output::*;

// Waits for a keypress and then handles it.
// Accepts input till Ctrl-q is pressed
pub fn editor_process_keypress() -> bool {
    let c = editor_read_key();

    match c {
        Ok(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        }) => true,
        _ => false,
    }
}

/*
--------------------My Version------------------------
pub fn editor_process_keypress() -> bool {
    let c = editor_read_key();

    if let Ok(c) = c {
        if c.code == KeyCode::Char('q') && c.modifiers.contains(KeyModifiers::CONTROL) { 
            return true;
//          std::process::exit(0);
        }
//        return true;
    }
    return false;
}
*/
