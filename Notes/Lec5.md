## Insert Ordinary Characters ( Step 101 - 103 )

Let's write a function to insert a character at a given position in a row.

```rust
// row.rs
    pub fn row_insert_char(&mut self, at: usize, c: char) {
        if at >= self.characters.len() {
            self.characters.push(c); }
        else {
            self.characters.insert(at, c);
        }
        self.render = Row::render_row(&self.characters)
    }

```

The function takes the row (self), position and the character as parameters. It adds the character at the end of the string if the position is past the string's length otherwise inserts the character at the position using the `insert()` function inbuilt for strings. Now, we update our `render` string to include this new character as well using `render_row()`. `row_insert_char()` modifies the associated variables with the row to have details of the text of the rows.

Now, let's deal with the cursor position to insert. Write a function `editor_insert_char()` as follows -

```rust
// editor.rs
    fn editor_insert_char(&mut self, c: char) {
        if self.cursor.y as usize == self.rows.len() {
            self.rows.push(Row::new(String::new()));
        }
        self.rows[self.cursor.y as usize].row_insert_char(self.cursor.x as usize, c);
        self.cursor.x += 1;
    }

```

We provide the character to be inserted as the parameter. If we are at the tilde line after the end of the file i.e checked by `self.cursor.y as usize == self.rows.len()`, then we append a new row before inserting the character. Then we call `row_insert_char()` on the row we are at with the column position as a paramter along with the character to insert. Also, we update the cursor's column position by 1, so that the next character is inserted after the previous insertion.

Now let's call this function for all the keys except the special keys (like ArrowUp, PageUp, Home, End etc.). But before that delete the 'wasd' key mappings, so that we can even insert these characters. Delete `keymap` and all its associated occurences (in `struct Editor`, `build()`, `process_keypress`). Thus, we don't need HashMap to store values now, so delete `use std::collections::HashMap`.

```rust
// editor.rs
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();
        
        if let Ok(c) = self.keyboard.read_key(){
            match c {
                /*...*/
                // Inserting characters
                KeyEvent {
                    code : KeyCode::Char(key),
                    modifiers : KeyModifiers::NONE,
                } => self.editor_insert_char(key),
		/*...*/
	   }
	}
     }

```

## Save the file

Let's first store our vector of `Row` structs in a single string.

```rust
// editor.rs
    fn row_to_string(&self) -> String {
        let mut data = String::new();

        for row in &self.rows {
            data.push_str(&row.characters);
            data.push('\n');
        }

        data
    }

```

We create a new empty string, and push each row to the string followed by a new line character at the end of the each row. But this will not work until and unless we make characters public. So, in the `struct Row`, change characters to `pub characters: String`. This will allow us access characters even beyond the file it is defined in.

Now we will write this string to the disk.

```rust
// editor.rs   
   fn save(&mut self) {
        let buf = self.row_to_string();
        let _ = std::fs::write(&self.filename, &buf)
    }

```

We call the `row_to_string()` function and store the string in a variable. Using `std::fs::write()`, we can save the content passed as the second parameter, with the filename passed as the first parameter.

`std::fs::write()` will create the file if it doesn't exists and will entirely replace its content if it does.

Now, let's map a key to this function. We will enable `Ctrl-s` to save it to the disk.

```rust
// editor.rs
pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();
        
        if let Ok(c) = self.keyboard.read_key(){
            match c {
		/*...*/	
                // Saving file
                KeyEvent {
                    code : KeyCode::Char('s'),
                    modifiers : KeyModifiers::CONTROL,
                } => self.save(),
		/*...*/
	    }
	}
}

```

Now, we'll be able to save the file to disk. Open a file, make changes, and press Ctrl-s to save followed by ctrl-q to exit. Reopen the file, and we will be able to see the saved changes.

Now, we will add some status message below the status bar, to let the user know if the save was successful or else display the error if it failed. Create a function to set the status message.

```rust
// editor.rs
    fn set_status_msg(&mut self, message: String) {
        self.status_time = Instant::now();
        self.status_msg = message;
    }

```

We set the `status_time` and `status_msg`. Now let's set a successful message if the file was properly written to the disk. Also, we'll check if the filename exists or not, if it doesn't then we'll just return and do nothing.

```rust
// editor.rs
    fn save(&mut self) {
        if self.filename.is_empty() {
            return;
        }
        
        let buf = self.row_to_string();
        let len = buf.as_bytes().len();
        if std::fs::write(&self.filename, &buf).is_ok() {
            self.set_status_msg(format!("{:?} bytes written to disk successfully", len));
        }
        else {
            self.set_status_msg(format!("Can't save! I/O error: {}", errno()));
        }

    }

```

We set the `len` as the total number of bytes written to the disk. And using `errno()` we'll display the associated error message. 

Also, now we should edit the status messgae displayed at the beginning to have information for saving the file along with quitting the application.

```rust
// editor.rs
    fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
    		/*...*/
    		status_msg : String::from("Help: Press Ctrl-q to exit | Ctrl-s to save"),
        	/*...*/
	})
    }
    
```

## Dirty Flag

Let's now let the user know if the file is modified or not, so they can save the changes before quitting. For this, let's create a variable `dirty` which will update each time we make some changes to the file. Let's define and intialise it first.

```rust
// editor.rs
pub struct Editor {
	/*...*/
	dirty: usize  
}

```

```rust
// editor.rs
fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
	/*...*/
	dirty: 0
	}

```

Now, let's update its value each time we make some change to the text.

```rust
// editor.rs
    fn editor_insert_char(&mut self, c: char) {
        if self.cursor.y as usize == self.rows.len() {
            self.append_row(String::new());
        }       
        self.rows[self.cursor.y as usize].row_insert_char(self.cursor.x as usize, c);
        self.cursor.x += 1;
        self.dirty += 1;
    }

```

```rust
// editor.rs
    fn append_row(&mut self, s: String){    
        self.rows.push(Row::new(s));
        self.dirty += 1;
    }

```

Now let the status bar display if the text is modified or not. If the value of dirty exceeds the initialized value, then the file has been changed otherwise not. We'll display this status after the filename. So, let's update the `left_text` which store the information to be stored at the left end of the status bar.

```rust
// editor.rs
pub fn refresh_screen(&mut self) -> Result<()> {
	/*...*/
	let left_txt = format!("{:20} {} - {} lines", 
                               if self.filename.is_empty(){"[No Name]"} else{&self.filename},
                                if self.dirty > 0{"(modified)"} else{""},
                                self.rows.len());
	/*...*/
}

```

We'll now be able to see "(modified)" in the status bar as soon as some change is made to the file. But we can observe that the "(modified)" doesn't appear even after we save it. This is because we have not reset its value while saving. So, let's do that!

```rust
// editor.rs
    fn save(&mut self) {
        if self.filename.is_empty() {
            return;
        }
       
        let buf = self.row_to_string();
        let len = buf.as_bytes().len();
        if std::fs::write(&self.filename, &buf).is_ok() {
            self.dirty = 0;
            self.set_status_msg(format!("{:?} bytes written to disk successfully", len));
        }
        else {
            self.set_status_msg(format!("Can't save! I/O error: {}", errno()));
        }

    }

```

## Quit Confirmation

Since we have make the user aware if the file is being modified or not, let's also warn if the user tries to quit without saving. So, to exit without saving, we ask user to press Ctrl-q three times. 

We first define `quit_times` and initialise it to three and then check if there are any changes made and also if Ctrl-q is pressed. We'll then warn the user to press it three more times, and decreament `quit_times` value each time.

```rust
// editor.rs
pub fn process_keypress(&mut self) -> Result<bool> {
	let bounds = self.screen.bounds();

	if let Ok(c) = self.keyboard.read_key(){
		match c {
			// Ctrl-q to exit
			KeyEvent {                          
				code: KeyCode::Char('q'),        
	      			modifiers: KeyModifiers::CONTROL,
			} => {
				if self.dirty > 0 && self.quit_times > 0 {
					self.set_status_msg(format!("Warning!! File has unsaved changes. \
								Press Ctrl-q {} more times to quit", self.quit_times));
					self.quit_times -= 1;
					return Ok(false);
				} 
				else {
					return Ok(true);
				}
			},
			/*...*/
		}
	}
	else {
		self.die("Unable to read from keyboard");
	}
	self.quit_times = KILO_QUIT_TIMES;
	Ok(false)
}

```

## Simple Backspacing

Let's implement backspace, Ctrl-h and Delete to remove characters from the screen. First we'll implement `del_char()` to return true if the character is deleted at a given position from the row and the row is updated as well, or else false.
```rust
// row.rs
    pub fn del_char(&mut self, at: usize) -> bool {
        if at >= self.characters.len() {
            false
        }
        else {
            self.characters.remove(at);
            self.render = Row::render_row(&self.characters);
            true
        }
    }

```

Now, let's delete the character that is to the left of the screen.

```rust
// editor.rs
    fn editor_del_char(&mut self) {
        if self.cursor.y as usize >= self.rows.len() {
            return;
        }
        
        if self.cursor.x > 0
            && self.rows[self.cursor.y as usize].del_char(self.cursor.x as usize - 1) {
                self.cursor.x -= 1;
                self.dirty += 1;
        }
        
    }

```

If the cursor position is beyond the rows in the file, we do nothing. We call our `del_char()` on the row we are at, for 1 left the position the cursor is currently at. We also shifts the cursor to the left and update `dirty` by 1, as there is one change made to file. Now, let's map Backspace and Ctrl-h keys to delete the character that is left to the cursor.

```rust
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();

        if let Ok(c) = self.keyboard.read_key(){
            match c {
		/*...*/

	        KeyEvent {
                    code : KeyCode::Backspace,
                    modifiers : NONE
                } => self.editor_del_char(),
     
                KeyEvent {
                    code : KeyCode::Char('h'),
                    modifiers : CONTROL
                } => self.editor_del_char(),
		/*...*/

```

Also, let `delete` key remove the character under the cursor, which can be treated as pressing a right arrow followed by backspace.

```rust
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();

        if let Ok(c) = self.keyboard.read_key(){
            match c {
                /*...*/
		KeyEvent {
                    code : KeyCode::Delete,
                    modifiers : NONE
                } => {
                        // Deletes the character under the cursor 
                        self.move_cursor(EditorKey::ArrowRight);
                        self.editor_del_char();
                    },

```
