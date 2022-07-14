use crossterm::Result;

mod screen;
mod keyboard;

mod editor;
use editor::*;

fn main() -> Result<()> { 
    
    let mut editor = Editor::new()?;

    editor.start()?;

    Ok(())  // Ok() represents success - return from a function
}


