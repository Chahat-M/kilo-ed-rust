use std::io::{stdout, Write, Stdout};

use crossterm::{QueueableCommand, style::Print, terminal, cursor, Result};

use kilo_ed_rust::*;

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
            height : rows,
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
    // Can check changes.rs
    pub fn draw_tildes(&mut self) -> Result<()>{
        for row in 0..self.height {
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            
            // Welcome msg along with tilde
            if row == self.height/3 {
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
        
       self.stdout.flush()
//       Ok(())
    }

    // Function to know the cursor position
    pub fn cursor_position(&self) -> Result<(u16, u16)> {
        cursor::position()
    }
    
    // Function to move the cursor to desired position
    pub fn move_to(&mut self, position : &CursorPos) -> Result<()> {
        self.stdout.queue(cursor::MoveTo(position.x, position.y))?;
        Ok(())
    }

}

