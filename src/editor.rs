use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

use std::io::{stdout, Write};

use crossterm::{QueueableCommand, terminal, cursor, Result};

use errno::errno;

use crate::screen::*;
use crate::keyboard::*;

use kilo_ed_rust::*;

pub struct Editor {
    screen : Screen,
    keyboard : Keyboard
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
        })
    }
    
    // Function to start the editor
    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;   // Step 5 - enabling raw mode during input

        loop{
            if self.refresh_screen().is_err(){
                self.die("Unable to refresh screen");
            }
            if self.process_keypress(){
                break;
            }
        }

        // To resolve error of Step 24
        let clear = if let Ok(clear) = self.screen.clear(){
            clear;
          };

        terminal::disable_raw_mode() // Step 6 - restoring the terminal mode after quitting
    }

    // Function to accept input till Ctrl-q is pressed
    // Waits for a keypress and then handles it.
    pub fn process_keypress(&mut self) -> bool {
        let c = self.keyboard.read_key();

        match c {
            Ok(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => true,
            Err(ResultCode::KeyReadFail) => {
                self.die("Unable to read from keyboard");
                false
            },
            _ => false,
        }
    }

    // Function to refresh the screen and move the cursor to top-left
    pub fn refresh_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();

        self.screen.clear()?;
        self.screen.draw_tildes()?;

        stdout.queue(cursor::MoveTo(0,0))?.flush()

    }

    // Function to exit the program
    pub fn die<S: Into<String>>(&mut self, message: S) {
        let _= self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }
}
