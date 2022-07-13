use crossterm::event::{read, Event::*, KeyCode, KeyModifiers, KeyEvent};

use std::io::{stdout, Write, Stdout};

use crossterm::{QueueableCommand, style::Print, terminal, cursor, Result};

use errno::errno;

#[derive(Debug)]
pub struct Editor {
    height : u16,
    width : u16,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width : columns,
            height : rows
        })
    }
    
    //From input.rs
    // Waits for a keypress and then handles it.
    // Accepts input till Ctrl-q is pressed
    pub fn process_keypress(&self) -> bool {
        let c = self.read_key();

        match c {
            Ok(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => true,
            _ => false,
        }
    }

    // From output.rs
    // To clear the screen and move the cursor to top left
    pub fn clear_screen(&self, stdout: &mut Stdout) -> Result<()>{

        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0,0))? // 1st(column, row)
            .flush()
    }

    // To draw Tildes(~) on the screen
    pub fn draw_tildes(&self, stdout: &mut Stdout) -> Result<()>{
        for row in 0..self.height {
            stdout
                .queue(cursor::MoveTo(0,row))?
                .queue(Print("~".to_string()))?;
            //println!("~\r");
        }

        Ok(())
    }

    // To refresh the screen and move the cursor to top-left
    pub fn refresh_screen(&self) -> Result<()> {
        let mut stdout = stdout();

        self.clear_screen(&mut stdout)?;
        self.draw_tildes(&mut stdout)?;

        stdout.queue(cursor::MoveTo(0,0))?.flush()

    }

    // Function for exiting the program
    pub fn die<S: Into<String>>(&self, message: S){
        let mut stdout = stdout();
        let _= self.clear_screen(&mut stdout);
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    // From keyboard.rs
    // Waits for one keypress and return it.
    pub fn read_key(&self) -> Result<KeyEvent> {
        loop{
            if let Ok(event) = read() {
                if let Key(key_event) = event{
                    return Ok(key_event);
                }
            } else {
                self.die("Read error");
                break;
            }
        }
        unreachable!();
    }

}
