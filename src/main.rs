use crossterm::Result;

mod screen;
mod keyboard;
mod row;

mod editor;
use editor::*;

fn main() -> Result<()> { 
    let mut args = std::env::args();

    // Condition to open file if passed or else open editor
    let mut editor = if args.len() >= 2 {
        Editor::open_file(args.nth(1).unwrap())?
    } else {
        Editor::new()?
    };

    editor.start()?;

    Ok(())  // Ok() represents success - return from a function
}


