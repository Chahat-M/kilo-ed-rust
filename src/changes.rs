//----------------My version for editor_refresh_screen() ----------------------------------

/*
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
// Version 2 -> Using ansi-escapes
// Works differently, needs to be checked and corrected
extern crate ansi_escapes;

pub fn editor_refresh_screen() {
    print!("{}", ansi_escapes::EraseScreen);
}
*/

//-------------------My version for draw_tildes() -----------------------------------------

/*
// To draw Tildes(~) on the screen
pub fn editor_draw_tildes(){
    for _row in 0..25{
        println!("~\r");
    }
}
*/

//----------------------My version for editor_process_keypress() --------------------------
/*
pub fn editor_process_keypress() -> bool {
    let c = editor_read_key();

    if let Ok(c) = c {
        if c.code == KeyCode::Char('q') && c.modifiers.contains(KeyModifiers::CONTROL) { 
            return true;
//          std::process::exit(0);
        }
//        return true;
    }
    return false;
}
*/
