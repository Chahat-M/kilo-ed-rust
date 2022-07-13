/*use crossterm::{
    event::{read, Event::Key, KeyCode, poll, KeyModifiers},
    terminal,
    Result};
*/

/*use crossterm::{event::Event::*, terminal, Result};

//use std::time::Duration;

mod keyboard;

mod input;
use input::*;

mod output;
use output::*;
*/

use crossterm::{terminal, Result};

mod editor;
use editor::*;

fn main() -> Result<()> { 
    
    let editor = Editor::new()?;
    println!("editor dimensions: {editor:?}");

    terminal::enable_raw_mode()?;   // Step 5 - enabling raw mode during input

    /*loop{
        if editor_refresh_screen().is_err(){
            die("Unable to refresh screen");
        }

        if editor_process_keypress(){
            break;
        }
    }*/
    
        loop{
        if editor.refresh_screen().is_err(){
            editor.die("Unable to refresh screen");
        }

        if editor.process_keypress(){
            break;
        }
    }

/*    loop{ 
        let mut c = None;

        match poll(Duration::from_millis(100)) {     // Timeout for read
            Ok(true) => {
                if let Ok(event) = read(){  // For reading an input
                    if let Key(key_event) = event{
                        c = Some(key_event);
                    }
                } else{
                    die("Read error");
                }
            }
            Ok(false) => {}
            _ => {
                die("Poll error");
            }
        }

        if let Some(c) = c {
            if c.code == KeyCode::Char('q') && c.modifiers.contains(KeyModifiers::CONTROL) { 
                break;
            } else{
                println!("{c:?}\r");  
                // \r indicates carriage return - moving cursor to beginning of line
            }
        } else{
            println!("no key\r");
        }
    }
*/

    terminal::disable_raw_mode()?;  // Step 6 - restoring the terminal mode after quitting

    Ok(())  // Ok() represents success - return from a function
}


