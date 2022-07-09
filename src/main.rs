use crossterm::{
    event::{read, Event::Key, KeyCode, poll, KeyModifiers},
    terminal,
    Result};

use std::time::Duration;

use errno::errno;

// Function to read a single event i.e char by char input
fn main() -> Result<()> { 
    //Result<T, E> is the type used for returning and propagating errors
    
    terminal::enable_raw_mode()?;   // Step 5 - enabling raw mode during input

    loop{ 
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
    terminal::disable_raw_mode()?;  // Step 6 - restoring the terminal mode after quitting

    Ok(())  // Ok() represents success - return from a function
}

// Function for exiting the program
pub fn die<S: Into<String>>(message: S){
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
