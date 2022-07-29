## A line Viewer (Step 55 - 60)

We intend to read a single line of text from a file and display it. For this let's begin by just displaying one line of text, we'll hardcode a "Hello, World" string and display it.

We'll modify our existing function `draw_tildes` to  accept a vector of strings as an argument and make further changes to display a line.

```
// editor.rs
pub fn draw_tildes(&mut self, erows : &[String]) -> Result<()>{
	for row in 0..self.height {
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		if row >= erows.len() as u16 {
			// Welcome msg along with tilde
			if row == self.height/3 {
				/*.../

				// Centering welcome msg with tildes
				/*
				if welcome.len() < self.width as usize {...}
				else {...}
				 */
			}

			// Tildes on all lines
			/*else {...}*/
		}

		// Opening the row
		else {
			let len = erows[0].len().min(self.width as usize);
			self.stdout
				.queue(cursor::MoveTo(0,row))?
				.queue(Print(erows[0][0..len].to_string()))?;
		}
	}

	self.stdout.flush()
}

```

We wrap our previous code in an if statement to check whether we are currently drawing a row that is the part of our text editor (containing tildes) or the row of the file. For now, our `erows.len()` will either be 0 or 1 indicating the no. of rows to display.

We add a new `else` section against the new `if` block to display the lines. We try that the text doesn't go past the end of the screen. And then, we move the cursor to the first column of the row (here the first row) and print the string charachter by charachter. 

To test this, let's change our editor.rs. We need to add a new `row : vec[String]` to define the row i.e the vector of strings in the `struct editor`. Also, we need to declare the string "Hello, World" under the `new()` function as shown - 

```
// editor.rs
pub fn new() -> Result<Self> {
	// Inserting cursor movements to HashMap
	/*...*/

	Ok(Self {
	    screen : Screen::new()?,
	    keyboard : Keyboard {},
	    cursor : CursorPos::default(),  // Initially - at default position
	    keymap,
	    rows : vec!["Hello World!".to_string()]
	})
}
```
Now, call the function py passing the new arguments as `draw_tildes(&self.rows)`, where it originally resides i.e in refresh_screen().

We have displayed a single line of text, so now let's open a file and read its first line only.

We create a new function `open_file` to open and read from the file. Our intention is to print the first line itslef, hence we modify the function as -

```
// editor.rs
pub fn open_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
	let first_line = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .next()
            .unwrap() 
            .to_string();
        Editor::build(&[first_line])
}
```

Using `read_to_string` method we read the file passed as an input and raise an error if it's unable to open. To identify every line, we split the lines by matching it with next line character '\n'. But for these functions to work, we need to import the external Path struct, by `use std::path::Path`, to read the path of the file. Also, the first line obtained is a string which is passed as the reference of the vector containing it, to our new function `build()`, which is later explained in detail.

Rename the new() function to build() that accepts a vector of string. We use vector of string because we intend to display multiple lines in future. Also, remove the hardcoded value to accept user value.

```
// editor.rs
fn build(data: &[String]) -> Result<Self>
```

```
// editor.rs
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : CursorPos::default(),  // Initially - at default position
            keymap,
            rows : if data.is_empty() { Vec::new() } else {Vec::from(data)}
        })
```
Specifying `rows` this way prevents us from printing any unnecessary blanks as defined in the new function below. This gives us our `build()` function. 

Now, we need to re-write the new() function that simply calls `build()`.

```
// editor.rs
pub fn new() -> Result<Self> {
	Editor::build("")
    }
```
Also, in `main()` we shall provide the condition to open the file if the input is provided else, to open our editor. If the length of the arguments passed are equal to or more than 2, indicates that the input file is provided. Hence we need to read the file.

```
// main.rs
    let mut args = std::env::args();

    // Condition to open file if passed or else open editor
    let mut editor = if args.len() >= 2 {
        Editor::open_file(args.nth(1).unwrap())?
    } else {
        Editor::new()?
    };
```
We are done with reading a line from a file!! But, there is something wrong! We don't want to show the welcome message when we open a file. So let's fix it.

We will print the welcome message only when the user starts the program with no arguments.

```
// editor.rs
    pub fn draw_tildes(&mut self, erows : &[String]) -> Result<()>{
        /*...*/
                if row == self.height/3 {
                if `erows.len() == 0` && row == self.height/3 {
		/*...*/
```

## Multiple lines (Step 61 - 65)

We have successfully read a single line from the file. Let's display all the contents of a file together. For this, we need to make some changes to our `open_file(filename)` function and to the `draw_tildes()` function as well.

```
// editor.rs
pub fn open_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let lines = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into())
	    .collect::<Vec<String>>();
        Editor::build(&lines)
}
```
Earlier we were just storing a single line, now we split the entire file content into separate lines, convert them into string using inbuilt `map()` function and store all the strings in a vector using `collect()`. Also, we rename `first_line` to `lines` for clear understanding.Since, build takes the reference to the vector as the parameter, we pass the reference to the vector lines. Let's update our `draw_tildes()` function accordingly.

```
// editor.rs
	/*...*/
	    // Printing the row
            else {
                let len = erows[row as usize].len().min(self.width as usize);
                self.stdout
                    .queue(cursor::MoveTo(0,row))?
                    .queue(Print(erows[row as usize][0..len].to_string()))?;
            }

```
Since we print each line, we print for every row character by character. But there is an extra blank line printed after the file, we will deal with this in future.

## Vertical Scrolling ( Step 66 - 70 )

Till now we can view the file, but if the file is goes beyond the screen height, we are unable to scroll through it. So, let's enable vertical scrolling. Create a variable `rowoff` to keep track of the row of the file the user is currently at and initialize it as 0, i.e scrolled at the top of the file by default.

```
// editor.rs
pub struct Editor {
    screen : Screen,
    keyboard : Keyboard,
    cursor : CursorPos,
    keymap : HashMap<char, EditorKey>,
    rows : Vec<String>,
    rowoff : u16
}

```

```
// editor.rs
	/*...*/
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : CursorPos::default(),  // Initially - at default position
            keymap,
            rows : if data.is_empty() { Vec::new() } else {Vec::from(data)},
	    rowoff : 0
        })
	/*...*/
```

There are two possibilities for the cursor to be at - above the visible window and below the visible window. If the cursor is at above the dispalyed screen then let's scroll up to where the cursor is. Or if the cursor is below the displayed screen, we change the `rowoff` value. Let's perform these in a `scroll()` function.

```
// editor.rs
/*...*/
fn scroll(&mut self) {
	let bounds = self.screen.bounds();
	if self.cursor.y < self.rowoff {
		self.rowoff = self.cursor.y; }
	if self.cursor.y >= self.rowoff + bounds.y {
		self.rowoff = self.cursor.y - bounds.y + 1; }
}

```

Now, let's display the correct range of lines of the file according to the value of `rowoff`.So we define the `filerow` variable to get the row of the file. 

```
// screen.rs
pub fn draw_tildes(&mut self, erows : &[String], rowoff : u16) -> Result<()>{
	for row in 0..self.height {
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            let filerow = (row + rowoff) as usize;
            if filerow >= erows.len() {
	/*...*/
```

Call the `scroll()` function right before we refresh screen.

```
// editor.rs
pub fn refresh_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();

        self.scroll();
        self.screen.clear()?;
        self.screen.draw_tildes(&self.rows, self.rowoff)?;
        self.screen.move_to(&self.cursor)?;

        stdout.flush()
}
```

Let's allow the cursor to move past the bottom of the screen but not past the bottom of the file.

```
// editor.rs
fn move_cursor(&mut self, key : EditorKey) {
	use EditorKey::*;

	match key {
		/*...*/
		ArrowDown => if (self.cursor.y as usize) < self.rows.len() { 
				  self.cursor.y += 1; }
	}
}
```

We are now able to scroll down the screen, but when we scroll up, the cursor isn't being positioned properly. This is because `self.cursor.y` now refers to the cursor position within the text file instead of the cursor position on the screen. For this, let's update our `move_to` function's definition and make necessary changes.

```
// screen.rs
pub fn move_to(&mut self, position : &CursorPos, rowoff : u16) -> Result<()> {
	self.stdout.queue(cursor::MoveTo(position.x, position.y - rowoff))?;
	Ok(())
}

```

```
// editor.rs
pub fn refresh_screen(&mut self) -> Result<()> {
	/*...*/
	self.screen.move_to(&self.cursor, self.rowoff)?;

	stdout.flush()
}
```

We are now done with enabling scrolling down and up the screen.

## Horizontal Scrolling

We'll perform similar steps as vertical scrolling. Define and initlize coloff in the same way as rowoff. Let's add the if statements to the `scroll` function, just by replacing `self.cursor.y` with `self.cursor.x`, `self.rowoff` with `self.coloff` and `bounds.y` with `bounds.x`.

```
// editor.rs
fn scroll(&mut self) {
        // Vertical scrolling
        /*...*/

        // Horizontal scrolling
        if self.cursor.x < self.coloff {
            self.coloff = self.cursor.x; }
        if self.cursor.x >= self.coloff + bounds.x {
            self.coloff = self.cursor.x - bounds.x + 1; }

}
```

Let's change our `draw_tildes` as per the requirements.

```
// screen.rs
 pub fn draw_tildes(&mut self, erows: &[String], rowoff: u16, coloff: u16) -> Result<()>{
	/*...*/
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
```

Here we calculate the length of the row and if it's less than the column offset value, we don't do anything and move to the next iteration, otherwise we subtract the entire length from the coloff value to get the remaining length. So, to scroll through the row, our start value becomes the coloff value. For the end value, we add the width of the screen to the start value if the remaining length is more, or else we simply add the remaining length. And now the range for the particular row of file changes to start to end.

Also, let's correct the way we call this function under `refresh_screen`, by passing a third argument `self.coloff`. Now, we'll be able to scroll horizontally as well!!

But let's fix the cursor positioning as we did in vertical scrolling. 

```
// screen.rs
pub fn move_to(&mut self, position: &CursorPos, rowoff: u16, coloff: u16) -> Result<()> {
	self.stdout.queue(cursor::MoveTo(position.x - coloff, position.y - rowoff))?;
	Ok(())
}
```

## Limit scrolling to the right

Currently we can scroll vertically and horizontally through the screen. But we don't want the user to move the cursor way off to the right of a line, thus we would allow the user to be one past the last charachter of the line, and one past the last line of the file.

```
// editor.rs
fn move_cursor(&mut self, key : EditorKey) {
	use EditorKey::*;

	let row_index = if self.cursor.y as usize >= self.rows.len() {
		None }
	else {
		Some(self.cursor.y as usize) };

	match key {
		/*...*/
		ArrowRight => { 
			if let Some(idx) = row_index {
				if (self.cursor.x as usize) < self.rows[idx].len() {
					self.cursor.x += 1; };
			}
		},
		/*...*/
	}
}
```

We check if the cursor is on an actual line, if it then we swt the `row_index` to the row the cursor is on. Also, we'll check if for that row the cursor is to the left of the end of the line, then only we increment it, and allow the cursor to move one past to the right of the end of line only.

## Snap cursor to end of line

The user is able to move the cursor till the past of the line, but they can still go right way too off by moving the cursor to the end of a long line and then down to the next line which is shorter. We need to fix this!

```
// editor.rs
fn move_cursor(&mut self, key : EditorKey) {    
	/*...*/

        match key {
                /*...*/
        }

	let rowlen = if self.cursor.y as usize >= self.rows.len() {
		0 }
	else {
		self.rows[self.cursor.y as usize].len() };

        self.cursor.x = self.cursor.x.min(rowlen as u16);
}
```

We check the length of the row the cursor is currently on and set it as `rowlen`. And we allow the cursor to be at the left of the line only.
