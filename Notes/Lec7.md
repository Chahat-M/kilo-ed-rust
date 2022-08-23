## Colorful Digits ( Step 142 )

Let's represent the digits in the text with red, just to add some colors on the screen. For using colors, we can import several structs from style module of crossterm crate, as per our need. Here we will just set the foreground color to make the digits red hence we will import `SetForegroundColor` struct.

```rust
// screen.rs
use crossterm::{
    QueueableCommand, 
    style::{Print, Color, Colors, SetColors, ResetColor, SetForegroundColor},
    terminal,
    cursor,
    Result};

```

Now, let's edit our `draw_tildes` function.

```rust
// screen.rs
pub fn draw_tildes(&mut self, erows: &[Row], rowoff: u16, coloff: u16) -> Result<()>{
	for row in 0..self.height {
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		let filerow = (row + rowoff) as usize;
		if filerow >= erows.len() {
			/*...*/
		}
		else {
			let mut len = erows[filerow].render_length();
			if len < coloff as usize {
				continue; }
			len -= coloff as usize;
			let start = coloff as usize;
			let end = start
				+ if len > self.width as usize {
					self.width as usize }
			else {
				len };

			self.stdout.queue(cursor::MoveTo(0,row))?;

			for ch in erows[filerow].render[start..end].to_string().chars() {
				if ch.is_digit(10) {
					self.stdout
						.queue(SetForegroundColor(Color::Red))?
						.queue(Print(ch))?
						.queue(SetForegroundColor(Color::Reset))?;
				} else {
					self.stdout
						.queue(Print(ch))?;                                              
				}                                                                        
			}                                                                            
		}   
	}
	self.stdout.flush()
}

```

Here, we just add the `for` block. Now we'll have to traverse each character of the `render` and check if it's a digit or not. If it's a digit we first set its color to red using `SetForegroundColor(Color::Red)` and then print the character. But before this we need to set our cursor to the starting of the screen. So add the `cursor::MOveTo()` line just before the for block. Also, we bring back the original color after each digit, using `SetForegroundColor(Color::Reset)` so that other non-digit characters are printed normally. 
