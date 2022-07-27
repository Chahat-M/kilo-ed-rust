## A line Viewer (Step 55 - 60)

We intend to read a single line of text from a file and display it. For this let's begin by just displaying one line of text, we'll hardcode a "Hello, World" string and display it.

We'll modify our existing function `draw_tildes` to  accept a vector of strings as an argument and make further changes to display a line.

```
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
fn build(data: &[String]) -> Result<Self>
```

```
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
pub fn new() -> Result<Self> {
	Editor::build("")
    }
```
Also, in `main()` we shall provide the condition to open the file if the input is provided else, to open our editor. If the length of the arguments passed are equal to or more than 2, indicates that the input file is provided. Hence we need to read the file.

```
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
    pub fn draw_tildes(&mut self, erows : &[String]) -> Result<()>{
        /*...*/
                if row == self.height/3 {
                if `erows.len() == 0` && row == self.height/3 {
		...
```

## Multiple lines (Step 61 - 65)

We have successfully read a single line from the file. Let's display all the contents of a file together. For this, we need to make some changes to our `open_file(filename)` function and to the `draw_tildes()` function as well.

```
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


