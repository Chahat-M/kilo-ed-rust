use std::io::{stdout, Write, Stdout};

use crossterm::{QueueableCommand, style::Print, terminal, cursor, Result};

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
    
    // To clear the screen and move the cursor to top left
    pub fn clear(&mut self) -> Result<()>{

        self.stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0,0))? // 1st(column, row)
                .flush()
    }

    // To draw Tildes(~) on the screen
    pub fn draw_tildes(&mut self) -> Result<()>{
        for row in 0..self.height {
            self.stdout
                    .queue(cursor::MoveTo(0,row))?
                    .queue(Print("~".to_string()))?;
            //println!("~\r");
        }
        
        self.stdout.flush()
//        Ok(())
    }
}

