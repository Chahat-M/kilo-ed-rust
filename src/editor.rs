use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

//use crossterm::event::{Event::Mouse, MouseEventKind, MouseEvent};

use std::io::{stdout, Write};

use crossterm::{QueueableCommand, terminal, cursor, Result};

use errno::errno;

use crate::screen::*;
use crate::keyboard::*;

use kilo_ed_rust::*;

use std::collections::HashMap;

use std::path::Path;

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
    keymap : HashMap<char, EditorKey>,
    rows : Vec<String>,
    rowoff : u16
}

impl Editor {
    // Function to open and read first line if the filename is passed
    pub fn open_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let lines = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into()) 
            .collect::<Vec<String>>();
        Editor::build(&lines)
    }

    pub fn new() -> Result<Self> {
        Editor::build(&[])
    }

    fn build(data: &[String]) -> Result<Self> {
        // Inserting cursor movements to HashMap
        let mut keymap = HashMap::new();
        keymap.insert('w', EditorKey::ArrowUp);
        keymap.insert('s', EditorKey::ArrowDown);
        keymap.insert('a', EditorKey::ArrowLeft);
        keymap.insert('d', EditorKey::ArrowRight);
    
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : CursorPos::default(),  // Initially - at default position
            keymap,
            rows : if data.is_empty() { Vec::new() } else {Vec::from(data)}, // Useful during hiding welcome msg
            rowoff : 0
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
/*            if let Ok(event) = crossterm::event::read() {
                match event {
                    Mouse(me) => match me.kind{
                        MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                            for _ in 0..bounds.y {
                                self.move_cursor( 
                                    if me.kind == MouseEventKind::ScrollUp 
                                        {EditorKey::ArrowUp}
                                    else {EditorKey::ArrowDown} )
                            }
                        },
                        _=> {}
                    }
                }
            }
*/            self.die("Unable to read from keyboard");
        }
        Ok(false)
    }

    // Function to refresh the screen and move the cursor to top-left
    pub fn refresh_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();
        
        self.scroll();
        self.screen.clear()?;
        self.screen.draw_tildes(&self.rows, self.rowoff)?;
        self.screen.move_to(&self.cursor, self.rowoff)?;
        
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
            ArrowDown => if (self.cursor.y as usize) < self.rows.len() { 
                            self.cursor.y += 1; }
        }
    }

    // Function for scrolling 
    fn scroll(&mut self) {
        let bounds = self.screen.bounds();
        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y; }
        if self.cursor.y >= self.rowoff + bounds.y {
            self.rowoff = self.cursor.y - bounds.y + 1; }
    }
}
