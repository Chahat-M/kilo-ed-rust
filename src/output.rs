use std::io::{stdout, Write, Stdout};

use crossterm::{QueueableCommand, style::Print, terminal, cursor, Result};

use errno::errno;

// To clear the screen and move the cursor to top left
pub fn clear_screen(stdout: &mut Stdout) -> Result<()>{
    
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0,0))? // 1st(column, row)
        .flush()
}

/*
----------------My Version--------------------------------
// To clear screen and move cursor to top left
use std::io::{stdout, Write};

use crossterm::{
    ExecutableCommand, terminal, Result };

pub fn editor_refresh_screen() -> Result<()> {
    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?;
        .execute(cursor::MoveTo(0,0))?;

    stdout.flush()?;
    Ok(())
}
*/

/*
-------------------My Version (Using ansi-escapes)-------------------
// Works differently, needs to be checked and corrected
extern crate ansi_escapes;

pub fn editor_refresh_screen() {
    print!("{}", ansi_escapes::EraseScreen);
}
*/


// To draw Tildes(~) on the screen
pub fn editor_draw_tildes(stdout: &mut Stdout) -> Result<()>{
    for row in 0..25{
        stdout
            .queue(cursor::MoveTo(0,row))?
            .queue(Print("~".to_string()))?;
        //println!("~\r");
    }

    Ok(())
}

/*
--------------------My version------------------------------
// To draw Tildes(~) on the screen
pub fn editor_draw_tildes(){
    for _row in 0..25{
        println!("~\r");
    }
}
*/

// To refresh the screen and move the cursor to top-left
pub fn editor_refresh_screen() -> Result<()> {
    let mut stdout = stdout();

    clear_screen(&mut stdout)?;
    editor_draw_tildes(&mut stdout)?;

    stdout.queue(cursor::MoveTo(0,0))?.flush()

}


// Function for exiting the program
pub fn die<S: Into<String>>(message: S){
    let mut stdout = stdout();
    let _= clear_screen(&mut stdout);
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}

