## Moving left at the start of a line ( Step 78 )

Let's allow the user to press the left arrow in a line to reach the end of the previous line.

```rust
// editor.rs
fn move_cursor(&mut self, key : EditorKey) {
	/*...*/
	match key {
		ArrowLeft => { 
			if self.cursor.y > 0 { 
				self.cursor.y = self.cursor.y.saturating_sub(1);
				self.cursor.x = self.rows[self.cursor.y as usize].len() as u16
			}
			else {
				self.cursor.x = self.cursor.x.saturating_sub(1)
			}
		}, 
			  /*...*/
	}
}
```

## Moving right at the end of the line ( Step 79 )

Let's allow the user to press reach the beginning of the next line when right arrow is pressed at the end of a line.

```rust
// editor.rs
fn move_cursor(&mut self, key : EditorKey) {
	/*...*/
	match key {
		/*...*/
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
			   /*...*/
	}
}
```

## Scrolling using `PageUp` and `PageDown` ( Step 90 )

Let's allow the user to reach at the top of the next page by pressing PageUp and at the end of the next page by pressing PageDown. If the page has less rows then the entire screen, then the cursor is placed at the end of the file. So, lets edit out `process_keypress` function.

```rust
// editor.rs
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
```

## Move to the End of the line with `End` ( Step 91 )

Earlier, we reached the end of the row of the screen on pressing `End`. But now, let's change `process_keypress` function to reach the end of the line of the file instead of the end of the screen.

```rust
// editor.rs
KeyCode::End =>
	if self.cursor.y < self.rows.len() as u16 {
        	self.cursor.x = self.rows[self.cursor.y as usize].len() as u16; },

```

## Tabs and Cursor ( Step 80 - 89 )

The cursor doesn't interact properly with tabs. We can even check this by creating a tabs.txt file that contains some text with tabs as well. We may notice that when there are multiple ttabs, the cursor skips it and moves to the next line, if any. To resolve this let's renders tab as multiple space characters.

To do so, create a `struct Row` that holds chars and render as strings. Implement the Row and define a new() function that takes the characters of the file as an argument and add these codes.

```rust
// screen.rs

const KILO_TAB_STOP: usize = 8;

pub Struct Row {
    chars: String,
    render: String,
}   

impl Row {
    pub fn new(chars: String) -> Self {
        let mut render = String::new();
        let mut index = 0;
        
        for c in chars.chars() {
            match c {
                '\t' => {
                    render.push(' ');
                    idx += 1;
                    while idx % KILO_TAB_STOP != 0 {
                        render.push(' ');
                        idx += 1;
                    }   
                }   
                _ => {
                    render.push(c); 
                    idx += 1;
                }
            }   
        }
        Self {chars, render}
    }
}

```

Firstly, set the tab stop as 8 by defining `const KILO_TAB_STOP: usize = 8`, so that we can easily use and play with it later.

Now, in the `new()` function we are initalising the render as an empty string for now. We loop through the charachters of the string passed as an argument to `new()`. If the charachter is a tab `\t` we append one space through `render.push()` (because each tab must advance the cursor forward at least one column), and then append spaces until we get to a tab stop, which is a column that is divisible by 8 (KILO_TAB_STOP). Then we return chars and render.

**To be continued...Doubt?**

## Status Bar ( Step 92 - 96 )

Before we get into text editing, let's display the status bar as well. It will show the information such as filename, how many lines are in file and what position your cursor is currently on. So, let's first subtract 1 from the height of the screen, so that there is no line at the bottom, where we will display our status bar.

```rust
// screen.rs
pub fn new() -> Result<Self> {
	let (columns, rows) = crossterm::terminal::size()?;
	Ok(Self {
		width : columns,
		height : rows - 1,
		stdout : stdout()
	})
}

```

To make the status bar stand out, display it with different colors. Here, we are displaying the background with DarkMagenta and forground with White. We use crossterm to set and reset colors.

```rust
// screen.rs
use crossterm::{style::{Print, Color, Colors, SetColors, ResetColor}}
```

```rust
// screen.rs
    pub fn draw_status_bar(&mut self) -> Result<()> {
        self.stdout
            .queue(cursor::MoveTo(0, self.height))?
            .queue(SetColors(Colors::new(Color::White, Color::DarkMagenta)))?
            .queue(Print(format!("{:01$}","",self.width as usize)))?
            .queue(ResetColor)?;
        Ok(())
    }

```

We move to the last row to display the status bar and set our desired colors. ALos, we print spaces for the entire screen width so that we have the entire status bar, and then we reset the terminal to its default colors.

Now we intend to display some information like filename, total lines, current cursor positionand percentage of the screen we are currently at, in our status bar. First we'll write the code and then understand it.

```rust
// screen.rs
pub fn draw_status_bar<T: Into<String>>(&mut self, left: T, right: T) -> Result<()> {
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
		.queue(ResetColor)?;
	Ok(())
}

```

We'll pass the text to be dispalyed at left end and at right end to the `draw_status_bar()` function as arguments. In the first two lines, we just convert them to the correct datatype. Then we store their lengths in separate variable for easier use. We define `lstatus` where we store our left string and `rstatus` where we will be storing our right string. For right string, we need to know the right edge of the screen, and thus we are not storing it simply.

Loop till we reach the end of the screen and check if there is just sufficient space for the right string or else just push space to `rstatus` and increament len by 1. Also, change the Print statement before ResetColor to now print the `lstatus` followed by `rstatus`.

Now, we'll define the left and right and call the function under `refresh_screen`. But before that we shall store the filename that we intend to display at the left end.

Create a new field filename under the `struct Editor` and initialize it.

```rust
//editor.rs
pub struct Editor {
	/*...*/
	filename: String
}

```

```rust
//editor.rs
fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
	/*...*/

	Ok(Self {
	/*...*/
	filename: filename.into()
	})
}

```

Since, the arguments of build function has changed we should change it in `new()` and `open_file()` as well.

```rust
//editor.rs
pub fn new() -> Result<Self> {
	Editor::build(&[],"".to_string())
}

```

```rust
//editor.rs
pub fn open_file<P: AsRef<Path> + ToString>(filename: P) -> Result<Self> {
	let fn_filename = filename.to_string();
	let lines = std::fs::read_to_string(filename)
		.expect("Unable to open file")
		.split('\n')
		.map(|x| x.into()) 
		.collect::<Vec<String>>();
	Editor::build(&lines, fn_filename)
}

```

Now we are ready to define left text `left_txt` and right text `right_txt` under `refresh_screen` below the we draw the rows i.e below `draw_tildes` call. 

In the left side we plan to display the filename and total no. of lines. The first 20 charachter of the filename will only be displayed, so files with very long names will be disaplyed till 20 charachters only. Also, we'll diaply total lines in the file after filename.

```rust
// editor.rs
let left_txt = format!("{:20} - {} lines", self.filename, self.rows.len());

```

In the right end we wish to display the current cursor position and percentage of the screen we are at. We'll define a new function `calc_percent()` to have all requirements for right text and return the right text so that we can simply call it. This is done to keep the `refresh_screen()` clean and clear to understand.

So, let's calculate percent as the current row we are at multiplied by 100 and divided by total rows. But if no arguments are passed i.e no file to open, there will be 0 rows, and it will result in panic! Thus, we'll check that condition. If there are no rows, we'll display "All" instaed of the percent. Also, we set conditions to display "TOP" if we are under 5% and "BOT" if we are past 95%, instead of the percent value.

```rust
// editor.rs
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

```

Now, to display the status bar call the function with `left_txt` and `right_txt` arguments.

```rust
// editor.rs
pub fn refresh_screen(&mut self) -> Result<()> {
	/*...*/
        self.screen.draw_tildes(&self.rows, self.rowoff, self.coloff)?;
	
	let left_txt = format!("{:20} - {} lines", self.filename, self.rows.len());
	let right_txt = self.calc_percent();
	self.screen.draw_status_bar(left_txt, right_txt)?;
	/*...*/
}
```

Be careful, call `draw_status_bar()` only after drawing rows to the screen i.e after `draw_tildes`.

## Status Message ( Step 97 - 100 )

We intend to display some useful information like how to exit etc. just below the status bar. Firstly, create and initialize fields under `struct Editor` to store the message and time. To store time we will import some `Duration` and `Instant` struct. We'll understand their use further.

```rust
// editor.rs
use std::time::{Instant, Duration};

```

```rust
// editor.rs
pub struct Editor {
    /*...*/
    status_time: Instant,
    status_msg: String,
}
```

```rust
// editor.rs
fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
	/*...*/
	Ok(Self {
	    /*...*/
            status_time: Instant::now(),
            status_msg: String::from("Help: Press Ctrl-q to exit")
        })
}
```

For now we just provide the meassage to quit from the editor. To dispaly this message below the status bar, firstly subtract 2 from the total rows in the screen, so the file rows will not be displayed in the last two rows. We'll make changes to `draw_status_bar()` to append one line after the status bar is drawn and to display the message as recieved in the parameters.

```rust
// screen.rs
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width : columns,
            height : rows - 2, // So that we can have status bar
            stdout : stdout()
        })
    }
```

```rust
// screen.rs
pub fn draw_status_bar<T: Into<String>>(&mut self, left: T, right: T, msg: String) -> Result<()> {
	/*...*/
        self.stdout
            .queue(cursor::MoveTo(0, self.height))?
            .queue(SetColors(Colors::new(Color::White, Color::DarkMagenta)))?
            .queue(Print(format!("{lstatus}{rstatus}")))?
            .queue(ResetColor)?
            .queue(cursor::MoveTo(0, self.height + 1))?
            .queue(Print(format!("{msg}")))?;

        Ok(())
    }

```

And now it's time for the final step before we move to text editing. We will the display the message below the status bar and clear it after 5 seconds of pressing any key. 

```rust
// editor.rs
pub fn refresh_screen(&mut self) -> Result<()> {
        if !self.status_msg.is_empty() && self.status_time.elapsed() > Duration::from_secs(5) {
            self.status_msg.clear();
        }

        self.screen.draw_status_bar(left_txt, right_txt, self.status_msg.to_string())?;
}

```

Hurray!! We have successfully built a text viewer and have reached half way. And it's time to change the text viewer to a text editor, enabling the user to insert, delete and save.
