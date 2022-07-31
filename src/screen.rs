use std::io::{stdout, Write, Stdout};

use crossterm::{
    QueueableCommand, 
    style::{Print, Color, Colors, SetColors, ResetColor},
    terminal,
    cursor,
    Result};

use kilo_ed_rust::*;

const KILO_TAB_STOP: usize = 8;

pub struct Screen {
    height : u16,
    width : u16,
    stdout : Stdout,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width : columns,
            height : rows - 2, // So that we can have status bar
            stdout : stdout()
        })
    }
    
    // Function to clear the screen and move the cursor to top left
    // Can check changes.rs
    pub fn clear(&mut self) -> Result<()>{

        self.stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0,0))? // 1st(column, row)
                .flush()
    }

    // Function to draw Tildes(~) on the screen
    // Alongwith welcome msg and rows
    // Can check changes.rs
    pub fn draw_tildes(&mut self, erows: &[String], rowoff: u16, coloff: u16) -> Result<()>{
        for row in 0..self.height {
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            let filerow = (row + rowoff) as usize;
            if filerow >= erows.len() {
                // Welcome msg along with tilde
                if erows.len() == 0 && row == self.height/3 {
                    let mut welcome = format!("Kilo Editor -- version {VERSION}");
                    welcome.truncate(self.width as usize);

                    // Centering welcome msg with tildes
                    if welcome.len() < self.width as usize {
                        let leftmost = (self.width - welcome.len() as u16)/2;
                        self.stdout
                            .queue(cursor::MoveTo(0,row))?
                            .queue(Print("~".to_string()))?
                            .queue(cursor::MoveTo(leftmost,row))?
                            .queue(Print(welcome))?;
                    }
                    else {
                        self.stdout
                            .queue(cursor::MoveTo(0,row))?
                            .queue(Print(welcome))?;
                    }
                }

                // Tildes on all lines
                else {
                    self.stdout
                        .queue(cursor::MoveTo(0,row))?
                        .queue(Print("~".to_string()))?;
                    /* For Step 40 - check it 
                       .queue(terminal::Clear(terminal::ClearType::UntilNewLine))?;
                       */
                    //println!("~\r");
                }
            }

            // Printing the row
            else {
                let mut len = erows[filerow].len();
                if len < coloff as usize {
                    continue; }
                len -= coloff as usize;
                let start = coloff as usize;
                let end = start 
                    + if len > self.width as usize {
                            self.width as usize }
                       else {
                            len };
                self.stdout
                    .queue(cursor::MoveTo(0,row))?
                    .queue(Print(erows[filerow][start..end].to_string()))?;
            }
        }
        
       self.stdout.flush()
//       Ok(())
    }

    // Function to know the cursor position
    pub fn cursor_position(&self) -> Result<(u16, u16)> {
        cursor::position()
    }
    
    // Function to move the cursor to desired position
    pub fn move_to(&mut self, position: &CursorPos, rowoff: u16, coloff: u16) -> Result<()> {
        self.stdout.queue(cursor::MoveTo(position.x - coloff, position.y - rowoff))?;
        Ok(())
    }

    // Function to know the height and width of the window
    pub fn bounds(&self)  -> CursorPos {
        CursorPos {
            x : self.width,
            y : self.height
        }
    }

    pub fn draw_status_bar<T: Into<String>>(
        &mut self, 
        left: T, 
        right: T, 
        msg: String) -> Result<()> {

        let left = left.into();
        let right = right.into();

        let left_width = left.len();
        let right_width = right.len();
        let screen_width = self.width as usize;

        let lstatus = format!("{left:0$}",left_width.min(screen_width));
        let mut rstatus = String::new();
        let mut len = lstatus.len();
        
        while len < screen_width {
            if screen_width - len == right_width {
                rstatus.push_str(right.as_str());
                break;
            }
            else {
                rstatus.push(' ');
                len += 1;
            }
        }
        
        self.stdout
            .queue(cursor::MoveTo(0, self.height))?
            .queue(SetColors(Colors::new(Color::White, Color::DarkMagenta)))?
            .queue(Print(format!("{lstatus}{rstatus}")))?
            .queue(ResetColor)?
            .queue(cursor::MoveTo(0, self.height + 1))?
            .queue(Print(format!("{msg}")))?;

        Ok(())
    }

}

