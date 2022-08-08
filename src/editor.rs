use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

//use crossterm::event::{Event::Mouse, MouseEventKind, MouseEvent};

use std::io::{stdout, Write};

use crossterm::{terminal, Result};

use errno::errno;

use crate::screen::*;
use crate::keyboard::*;
use crate::row::*;

use kilo_ed_rust::*;

use std::path::Path;

use std::time::{Instant, Duration};

const KILO_QUIT_TIMES: usize = 3;

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
    rows : Vec<Row>,
    rowoff : u16,
    coloff : u16,
    filename: String,
    status_time: Instant,
    status_msg: String,
    render_x: u16,
    dirty: usize,
    quit_times: usize
}

impl Editor {
    // Function to open and read first line if the filename is passed
    pub fn open_file<P: AsRef<Path> + ToString>(filename: P) -> Result<Self> {
        let fn_filename = filename.to_string();
        let lines = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into()) 
            .collect::<Vec<String>>();
        Editor::build(&lines, fn_filename)
    }

    pub fn new() -> Result<Self> {
        Editor::build(&[],"")
    }
    
    fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : CursorPos::default(),  // Initially - at default position
            rows : if data.is_empty() { Vec::new() } 
                else { 
                    let v = Vec::from(data);
                    let mut rows = Vec::new();
                    for row in v {
                        rows.push(Row::new(row));
                    }
                    if rows.last().unwrap().len() == 0{
                        rows.pop();
                    }
                    rows
                },
            rowoff : 0,
            coloff : 0,
            filename : filename.into(),
            status_time : Instant::now(), // Current time
            status_msg : String::from("Help: Press Ctrl-q to exit | Ctrl-s to save"),
            render_x : 0,
            dirty: 0, 
            quit_times: KILO_QUIT_TIMES
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
        let _ = self.screen.clear();

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
                } => {
                    if self.dirty > 0 && self.quit_times > 0 {
                        self.set_status_msg(format!("Warning!! File has unsaved changes. \
                        Press Ctrl-q {} more times to quit", self.quit_times)); 
                        self.quit_times -= 1;
                        return Ok(false);
                    } 
                    else {
                        return Ok(true);
                    }
                },
                
                // Inserting characters
                KeyEvent {
                    code : KeyCode::Char(key),
                    modifiers : KeyModifiers::NONE | KeyModifiers::SHIFT 
                } => self.editor_insert_char(key),

                // Saving file 
                KeyEvent {
                    code : KeyCode::Char('s'),
                    modifiers : KeyModifiers::CONTROL,
                } => self.save(),

                KeyEvent {
                    code : KeyCode::Backspace,
                    modifiers : KeyModifiers::NONE
                } => self.editor_del_char(),

                KeyEvent {
                    code : KeyCode::Char('h'),
                    modifiers : KeyModifiers::CONTROL
                } => self.editor_del_char(),

                KeyEvent {
                    code : KeyCode::Delete,
                    modifiers : KeyModifiers::NONE
                } => {
                        // Deletes the character under the cursor 
                        self.move_cursor(EditorKey::ArrowRight);
                        self.editor_del_char();
                    },
                
                KeyEvent {
                    code : KeyCode::Enter,
                    modifiers : KeyModifiers::NONE
                } => self.insert_new_line(),

                // Cursor movement through arrow keys
                KeyEvent { code, modifiers : _ } => match code {
                    KeyCode::Home => self.cursor.x = 0,
                    KeyCode::End => 
                        if self.cursor.y < self.rows.len() as u16 {
                        self.cursor.x = self.rows[self.cursor.y as usize].len() as u16; },
                    KeyCode::Up => self.move_cursor(EditorKey::ArrowUp),
                    KeyCode::Down => self.move_cursor(EditorKey::ArrowDown),
                    KeyCode::Left => self.move_cursor(EditorKey::ArrowLeft),
                    KeyCode::Right => self.move_cursor(EditorKey::ArrowRight),
                    KeyCode::PageUp | KeyCode::PageDown => {
                        if code == KeyCode::PageUp {
                            self.cursor.y = self.rowoff; }
                        else {
                            self.cursor.y = 
                                (self.rowoff + bounds.y - 1).min(self.rows.len() as u16); }
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
        self.quit_times = KILO_QUIT_TIMES;
        Ok(false)
    }

    // Function to refresh the screen and move the cursor to top-left
    pub fn refresh_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();
        
        self.scroll();
        self.screen.clear()?;
        self.screen.draw_tildes(&self.rows, self.rowoff, self.coloff)?;
        
        let left_txt = format!("{:20} {} - {} lines", 
                               if self.filename.is_empty(){"[No Name]"} else{&self.filename},
                                if self.dirty > 0{"(modified)"} else{""},
                                self.rows.len());

        let right_txt = self.calc_percent();
        
        if !self.status_msg.is_empty() && self.status_time.elapsed() > Duration::from_secs(5) {
            self.status_msg.clear();
        }

        self.screen.draw_status_bar(left_txt, right_txt, self.status_msg.to_string())?;
        

        self.screen.move_to(&self.cursor, self.render_x, self.rowoff, self.coloff)?;

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

       let row_index = if self.cursor.y as usize > self.rows.len() {
                            None }
                       else {
                            Some(self.cursor.y as usize) };

        match key {
            ArrowLeft => { 
                if self.cursor.x != 0  {
                    self.cursor.x = self.cursor.x.saturating_sub(1)
                }
                else if self.cursor.y > 0 { 
                    self.cursor.y = self.cursor.y.saturating_sub(1);
                    self.cursor.x = self.rows[self.cursor.y as usize].len() as u16
                }
            }, 
            ArrowRight => {
                if let Some(idx) = row_index {
                    if (self.cursor.x as usize) < self.rows[idx].len() {
                        self.cursor.x += 1; }
                    else if (self.cursor.x as usize) == self.rows[idx].len() {
                        self.cursor.y += 1;
                        self.cursor.x = 0
                    };
                }
            },
            //{ self.cursor.x = self.cursor.x.saturating_add(1) },
            ArrowUp => { self.cursor.y = self.cursor.y.saturating_sub(1) },
            ArrowDown => if (self.cursor.y as usize) < self.rows.len() { 
                            self.cursor.y += 1; }
        }

        let rowlen = if self.cursor.y as usize >= self.rows.len() {
                        0 }
                     else {
                         self.rows[self.cursor.y as usize].len() };

        self.cursor.x = self.cursor.x.min(rowlen as u16);
    }

    // Function for scrolling 
    fn scroll(&mut self) {
        let bounds = self.screen.bounds();
        
        self.render_x = if self.cursor.y < self.rows.len() as u16 {
            self.rows[self.cursor.y as usize].cursorx_to_renderx(self.cursor.x) }
        else {
            0 };

        // Vertical scrolling
        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y; }
        if self.cursor.y >= self.rowoff + bounds.y {
            self.rowoff = self.cursor.y - bounds.y + 1; }
        
        // Horizontal scrolling
        if self.render_x < self.coloff {
            self.coloff = self.render_x; }

        if self.render_x >= self.coloff + bounds.x {
            self.coloff = self.render_x - bounds.x + 1; }
    }
    
    fn calc_percent(&self) -> String {
        let percent = if self.rows.len() > 0 {
            (self.cursor.y as usize * 100)/self.rows.len() }
            else {
                0
            };
        
        let mut right_txt = format!("{},{}      {}%", self.cursor.y, self.cursor.x, percent);
 
        if self.rows.len() == 0 { 
                right_txt = format!("{},{}        All", self.cursor.y, self.cursor.x); }
        else if percent < 5 { 
                right_txt = format!("{},{}      TOP", self.cursor.y, self.cursor.x); }
        else if percent > 95 {
            right_txt = format!("{},{}      BOT", self.cursor.y, self.cursor.x);
        }
        
        right_txt
    }
    

    fn editor_insert_char(&mut self, c: char) {
        if self.cursor.y as usize == self.rows.len() {
            self.insert_row(self.rows.len(), String::new());
        }
        self.rows[self.cursor.y as usize].row_insert_char(self.cursor.x as usize, c);
        self.cursor.x += 1;
        self.dirty += 1;
    }

    fn insert_row(&mut self, at: usize, s: String){
        if at > self.rows.len() {
            return;
        }

        self.rows.insert(at, Row::new(s));
        self.dirty += 1;
    }

    fn row_to_string(&self) -> String {
        let mut data = String::new();

        for row in &self.rows {
            println!("{:?}", &row.characters);
            data.push_str(&row.characters);
            data.push('\n');
        }

        data
    }

    fn save(&mut self) {
        if self.filename.is_empty() {
            if let Some(filename) = self.prompt("Save as (ESC to cancel)"){
                self.filename = filename;
            } else {
                self.set_status_msg(String::from("Save aborted"));
                return;
            }
        }
       
        let buf = self.row_to_string(); 
        let len = buf.as_bytes().len();
        if std::fs::write(&self.filename, &buf).is_ok() {
            self.dirty = 0;
            self.set_status_msg(format!("{:?} bytes written to disk successfully", len));
        }
        else {
            self.set_status_msg(format!("Can't save! I/O error: {}", errno()));
        }

    }
    

    fn set_status_msg(&mut self, message: String) {
        self.status_time = Instant::now();
        self.status_msg = message;
    }

    // Function to delete character left of the cursor from the screen
    fn editor_del_char(&mut self) {
        if self.cursor.y as usize == self.rows.len() {
            return;
        }

        if self.cursor.x as usize == 0 && self.cursor.y == 0 {
            return;
        }

        if self.cursor.x > 0 
            && self.rows[self.cursor.y as usize].del_char(self.cursor.x as usize - 1) {
                self.cursor.x -= 1;
                self.dirty += 1;
        } else {
            self.cursor.x = self.rows[self.cursor.y as usize - 1].len() as u16;
            if let Some(row) = self.del_row(self.cursor.y as usize) {
                self.rows[self.cursor.y as usize - 1].append_string(&row);
                self.cursor.y -= 1;
                self.dirty += 1;
            }
        }
        
    }
    
    // Function to delete a row and return its contents
    fn del_row(&mut self, at: usize) -> Option<String>{
        if at >= self.rows.len() {
            None
        } else {
            self.dirty += 1;   
            Some(self.rows.remove(at).characters)
        }

    }

    fn insert_new_line(&mut self){
        if self.cursor.x == 0 {
            self.insert_row(self.cursor.y as usize, "".to_string());
        } else {
            let new_row = self.rows[self.cursor.y as usize].rowsplit(self.cursor.x as usize);
            self.insert_row(self.cursor.y as usize + 1, new_row);
        }
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    // Prompts the user if saves without filename
    fn prompt(&mut self, pmsg: &str) -> Option<String> {
        let mut buf = String::from("");

        loop {
            self.set_status_msg(format!("{}: {}", pmsg, buf));
            let _ = self.refresh_screen();
            if let Ok(c) = self.keyboard.read_key() {
                match c {                    
                    KeyEvent { 
                        code: KeyCode::Enter,
                        ..
                    } => {
                        self.set_status_msg("".to_string());
                        return Some(buf);
                    },

                    KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    } => { 
                        self.set_status_msg("".to_string());
                        return None;
                    },

                    KeyEvent {
                        code: KeyCode::Char(ch),
                        modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT
                    } => {
                        buf.push(ch);
                    },

                    KeyEvent {
                        code: KeyCode::Backspace | KeyCode::Delete,
                        ..
                    }
                    |
                    KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::CONTROL
                    } => {
                        buf.pop();
                    },

                    _=> {}

                }
            }
        }
    }
}
