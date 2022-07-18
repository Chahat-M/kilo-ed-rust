use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

use std::io::{stdout, Write};

use crossterm::{QueueableCommand, terminal, cursor, Result};

use errno::errno;

use crate::screen::*;
use crate::keyboard::*;

use kilo_ed_rust::*;

use std::collections::HashMap;

// Copy -> to give EditorKey Copy semantics instead of Move semantics
// Clone -> to create T from &T via a copy
// Types that are Copy should have a trivial implementation of Clone, hence both used.
#[derive(Copy, Clone)] 
enum EditorKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight
}

pub struct Editor {
    screen : Screen,
    keyboard : Keyboard,
    cursor : CursorPos,
    keymap : HashMap<char, EditorKey>
}

impl Editor {
    pub fn new() -> Result<Self> {

        let mut keymap = HashMap::new();
        keymap.insert('w', EditorKey::ArrowUp);
        keymap.insert('s', EditorKey::ArrowDown);
        keymap.insert('a', EditorKey::ArrowLeft);
        keymap.insert('d', EditorKey::ArrowRight);

        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : CursorPos::default(),  // Initially - at default position
            keymap
        })
    }
    
    // Function to start the editor
    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;   // Step 5 - enabling raw mode during input

        loop{
            if self.refresh_screen().is_err(){
                self.die("Unable to refresh screen");
            }
          
            if self.process_keypress()? {
                break;
            }
        }

        // To resolve error of Step 24
        let cl = if let Ok(cl) = self.screen.clear(){
            cl
        };

        terminal::disable_raw_mode() // Step 6 - restoring the terminal mode after quitting
    }

    // Function to accept input till Ctrl-q is pressed
    // Waits for a keypress and then handles it.
    // Can check changes.rs for own definition
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();
        
        if let Ok(c) = self.keyboard.read_key(){
            match c {
                // Ctrl-q to exit
                KeyEvent {
                    code: KeyCode::Char('q'),       
                    modifiers: KeyModifiers::CONTROL,
                } => return Ok(true),
                
                // Cursor movement through 'wasd'
                KeyEvent {
                    code : KeyCode::Char(key),
                    modifiers : _
                } => match key {
                    'w' | 'a' | 's' | 'd' =>{
                        let temp = *self.keymap.get(&key).unwrap();
                        self.move_cursor(temp);
                    },
                    _ => {}
                },

                // Cursor movement through arrow keys
                KeyEvent { code, modifiers : _ } => match code {
                    KeyCode::Home => self.cursor.x = 0,
                    KeyCode::End => self.cursor.x = bounds.x - 1,
                    KeyCode::Up => self.move_cursor(EditorKey::ArrowUp),
                    KeyCode::Down => self.move_cursor(EditorKey::ArrowDown),
                    KeyCode::Left => self.move_cursor(EditorKey::ArrowLeft),
                    KeyCode::Right => self.move_cursor(EditorKey::ArrowRight),
                    KeyCode::PageUp | KeyCode::PageDown => {
                        for _ in 0..bounds.y {
                            self.move_cursor( if code == KeyCode::PageUp {EditorKey::ArrowUp}
                                             else {EditorKey::ArrowDown} )
                        }
                    },
                    _ => {}
                },
            }
        }
        else {
            self.die("Unable to read from keyboard");
        }
        Ok(false)
    }

    // Function to refresh the screen and move the cursor to top-left
    pub fn refresh_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();

        self.screen.clear()?;
        self.screen.draw_tildes()?;
        self.screen.move_to(&self.cursor)?;
        
        stdout.flush()

//        stdout.queue(cursor::MoveTo(0,0))?.flush()

    }

    // Function to exit the program
    pub fn die<S: Into<String>>(&mut self, message: S) {
        let _= self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }
    
    // Function to allow cursor movement
    // Left a, Right d, Up w, Down s
    fn move_cursor(&mut self, key : EditorKey) {
       use EditorKey::*;

        match key {
            ArrowLeft => { self.cursor.x = self.cursor.x.saturating_sub(1) }, 
            ArrowRight => self.cursor.x += 1, 
            //{ self.cursor.x = self.cursor.x.saturating_add(1) },
            ArrowUp => { self.cursor.y = self.cursor.y.saturating_sub(1) },
            ArrowDown =>  self.cursor.y += 1
        }
    }
}
