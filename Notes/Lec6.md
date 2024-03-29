## The Enter Key ( Step 122 - 125 )

Now let's enable the Enter key as well. It will either insert a new row or split a line into two lines. Let's begin it by renaming `append_row()` to `insert_row()` with an additional argument `at` that defines the cursor position where we have to insert the text/row. The modified function becomes - 

```rust
// editor.rs
    fn insert_row(&mut self, at: usize, s: String){
        if at > self.rows.len() {
            return;
        }

        self.rows.insert(at, Row::new(s));
        self.dirty += 1;
    }

```

Now, let's also change the occurance of `append_row()` to `insert_row()` with proper parameters in `editor_insert_char()`

```rust
// editor.rs
    fn editor_insert_char(&mut self, c: char) {
        if self.cursor.y as usize == self.rows.len() {
            self.insert_row(self.rows.len(), String::new());
        } 
        self.rows[self.cursor.y as usize].row_insert_char(self.cursor.x as usize, c);
        self.cursor.x += 1;
        self.dirty += 1;
    }

```

Let's create a separate function to deal with insertion of a new line. We need to insert a blank line if we press Enter at the beginning of a row, or else if the cursor is somewhere between the text, we will have to split the text. The contents in the left of the cursor remains in the same line and the contents in the right moves to the next line.

```rust
// editor.rs
    fn insert_new_line(&mut self){
        if self.cursor.x == 0 {
            self.insert_row(self.cursor.y as usize, "".to_string());
        } else {
            let new_row = self.rows[self.cursor.y as usize].rowsplit(self.cursor.x as usize);
            self.insert_row(self.cursor.y as usize + 1, new_row);
        }
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

```

Here we have used `rowsplit(self.cursor.x as usize)`, which is unknown to us. We'll now define this function `rowsplit()` which takes the cursor position at which Enter will be pressed as the position. It will return the contents of the row that are at the right of the cursor i.e from the position specifiecd till the last character of the row.

```rust
// row.rs
    pub fn rowsplit(&mut self, from: usize) -> String {
        let next_row = self.characters.split_off(from);
        self.render = Row::render_row(&self.characters);
     
        next_row
    }

```

We have used the inbuilt `split_off()` function that separates the contents into two at the given index. The left half [0,from) is updated as self itself whereas the right half [from,len) is returned as string which is stored in `next_row`.

Now let's map the Enter key to `insert_new_line()` and run the program.

```rust
// editor.rs
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();

        if let Ok(c) = self.keyboard.read_key(){
            match c {
		/*...*/
		
		KeyEvent {
                    code : KeyCode::Enter,
                    modifiers : KeyModifiers::NONE
                } => self.insert_new_line(),
		
		/*...*/
	    }
	}
}

```

## Save As ( Step 126 - 130 )

Currently, the user is unable to save the file with a filename when they run the program without any arguments. Let's define a function `prompt()` that will prompt the user for filename and allow them to input the name if he/she exits without saving.

```rust
// editor.rs
    fn prompt(&mut self, pmsg: &str) -> String {
        let mut buf = String::from("");

        loop {
            self.set_status_msg(format!("{}, {}", pmsg, buf));
            let _ = self.refresh_screen();
            if let Ok(c) = self.keyboard.read_key() {
                match c {
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        self.set_status_msg("".to_string());
                        return buf;
                    },

                    KeyEvent {
                        code: KeyCode::Char(ch),
                        modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT
                    } => {
                        buf.push(ch);
                    },

                    _=> {}

                }
            }
        }
    }

```
We store the user input in buf. We run an infinite loop, and display the msg to the user to save and also refresh the screen in each iteration. When a keypress is read, we see if the user has typed a letter/number/special character or has pressed Enter. When enter is pressed, we clear the message below the status bar and return the input stored in buf. Or else if some other key is pressed, so we add the character to our string `buf`. 

Now, let's update `save()` so that if no filename is provided we can prompt the user.

```rust
// editor.rs
    fn save(&mut self) {
        if self.filename.is_empty() {
            self.filename = self.prompt("Save as: ");
        }  
    }

```

Let's now allow the user to press Esc to cancel the input. We'll check if the user has pressed Esc, and if so, we will print no message and return nothing. In Rust, to return nothing is equivalent to return None. So, we'll have to change the return type of `prompt` from `String` to `Option<String>` and thus changing the return input statement i.e from `return buf` to `return Some(buf)`.

```rust
// editor.rs
    fn prompt(&mut self, pmsg: &str) -> Option<String> {
        let mut buf = String::from("");

        loop {
            self.set_status_msg(format!("{}: {}", pmsg, buf));
            let _ = self.refresh_screen();
            if let Ok(c) = self.keyboard.read_key() {
                match c {
                    KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    } => { 
                        self.set_status_msg("".to_string());
                        return None;
                    },

                    KeyEvent { 
                        code: KeyCode::Enter,
                        ..
                    } => {
                        self.set_status_msg("".to_string());
                        return Some(buf);
                    },

		/*...*/
		}
	    }
	}
    }

```

Also, now let us know the user that the save is cancelled and also modify `save()` as per the updated return type of `prompt()`.

```rust
// editor.rs
    fn save(&mut self) {
        if self.filename.is_empty() {
            if let Some(filename) = self.prompt("Save as (ESC to cancel)"){
                self.filename = filename;
            } else {
                self.set_status_msg(String::from("Save aborted"));
                return;
            }
        }
	/*...*/
    }

```

Let's now enalble Backspace, Ctrl-h and Delete for the input prompt as well.

```rust
// editor.rs
    fn prompt(&mut self, pmsg: &str) -> Option<String> {
        let mut buf = String::from("");  
            
        loop {
            self.set_status_msg(format!("{}: {}", pmsg, buf));
            let _ = self.refresh_screen();
            if let Ok(c) = self.keyboard.read_key() {
                match c {
		    /*...*/
		    KeyEvent {
                        code: KeyCode::Backspace | KeyCode::Delete,
                        ..
                    }
                    |
                    KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::CONTROL
                    } => {
                        buf.pop();
                    },
		    /*...*/
		}
	   }
       }
    }

```

Hurray!! We are done with the basic text editor. We'll be able to open a file, make changes and save it. Also, we can create a new file, write what we wish and save it.

## Search ( Step 131 - 134 )

Let's now allow the user to search for a character or a string in the file. We'll start by defining a function `find()` that prompts the user for the query to search and places the cursor at the first starting index of the query.

```rust
// editor.rs
    fn find(&mut self) {
        if let Some(query) = self.prompt("Search (ESC to cancel)") {
           for (i, row) in self.rows.iter().enumerate() {
                if let Some(m) = row.characters.match_indices(query.as_str()).take(1).next() {
                    self.cursor.y = i as u16;
                    self.cursor.x = m.0 as u16;
                    self.rowoff = self.rows.len() as u16;
                    break;
                }
            }
        } 
    }

```

We allow the user to input the query and iterate over each row along with storing the iteration number in `i` and see if the character/string exists in that row. We match the rows' characters with the query using `match_indices(query)` which is inbuilt function for String and taking one at a time using `take(1)` and moving to the next value by `next()`. `match_indices()` return the starting index of the pattern followed by an iterator over the matches of the pattern. If we get a match, we set our cursor's vertical position in that row and the horizontal position at the starting index of the query.  Lastly, we set `self.rowoff` so that we are scrolled to the very bottom of the file, which will cause `scroll()` to scroll upwards at the next screen refresh so that the matching line will be at the very top of the screen. 

Let's now call this function when we press Ctrl-f and also update the status message to display this information at the start of the program.

```rust
// editor.rs
    pub fn process_keypress(&mut self) -> Result<bool> {
        let bounds = self.screen.bounds();

        if let Ok(c) = self.keyboard.read_key(){
            match c {
		/*...*/
		KeyEvent {
                    code : KeyCode::Char('f'),
                    modifiers : KeyModifiers::CONTROL,
                } => self.find(),
		/*...*/
	    }
	}
   }

```

```rust
// editor.rs
    fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
		/*...*/
		status_msg : String::from("Help: Press Ctrl-q to exit | Ctrl-s to save | Ctrl-f to find"),
		/*...*/
	})
    }

```

## Incremental Search ( Step 135 - 137 )

Now let's improve our search such that we find the match after each character is pressed. To implement this, we'll modify our `prompt()` to take a callback function as an argument. Also, let's call this function after each keypress, passing the current search query and the key pressed at last. For this we can define an `enum PromptKey` to hold the possible keypresses i.e Enter, Escape or any character. Let's declare this enum at the top of the file after all the import statements.

```rust
// editor.rs
enum PromptKey {
    Enter,
    Escape,
    Char(char)
}

```
Let's modify the `prompt()` as per the requirements above.

```rust
// editor.rs
    fn prompt(
        &mut self, 
        pmsg: &str, 
        callback: Option<fn(&mut Editor, &str, PromptKey)>) -> Option<String> {
        let mut buf = String::from("");
	/*...*/
        
	let mut prompt_key: Option<PromptKey> = None;

	match c {                    
	    KeyEvent { 
		code: KeyCode::Enter,
	      	..
	    } => {
		if let Some(callback) = callback {
			callback(self, &buf, PromptKey::Enter);
		}
		self.set_status_msg("".to_string());
		return Some(buf);
	    },

	    KeyEvent {
                code: KeyCode::Esc,
                ..
            } => { 
                if let Some(callback) = callback {
                        callback(self, &buf, PromptKey::Escape);
                }
                self.set_status_msg("".to_string());
                return None;
	    },

            KeyEvent {
	    	code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT
            } => {
	    	prompt_key = Some(PromptKey::Char(ch));
                buf.push(ch);
            },

	    _=> {}
	}
	if let Some(callback) = callback {
        	if let Some(key) = prompt_key {
                        callback(self, &buf, key);
                }
        }

```

Note - Explanation of None 

```rust
// editor.rs
    fn find(&mut self) {
        if let Some(query) = self.prompt("Search (ESC to cancel)", None) { 
	/*...*/
	}
    }

    fn save(&mut self) {
	if self.filename.is_empty() {
		if let Some(filename) = self.prompt("Save as (ESC to cancel)", None){
			self.filename = filename;
		}
		/*...*/
	}
    }

```

Now let's move the searching code of `find()` to another function `find_callback()`, which will be the callback argument for `prompt()`. Also, let's leave the search mode if the user presses Enter or Escape key.

```rust
// editor.rs
    fn find_callback(&mut self, query: &str, event: PromptKey) {
        if matches!(event, PromptKey::Enter | PromptKey::Escape) {
            return;
        }

        for (i, row) in self.rows.iter().enumerate() {
            if let Some(m) = row.characters.match_indices(query).take(1).next() {
                self.cursor.y = i as u16;
                self.cursor.x = m.0 as u16;
                self.rowoff = self.rows.len() as u16;
                break;
            }
        }
    }

```

```rust
// editor.rs
    fn find(&mut self) {
        self.prompt("Search (ESC to cancel)", Some(Editor::find_callback)); 
    }

```

Now we can observe the search results after each keypress. Also, on pressing Enter or Escape we leave the search mode.

## Restore cursor position on Escape ( Step 138 )

We want that on pressing Escape, the user shall go back to the position in the file where they started the search. For this, let's save the cursor's position and scroll position before the search and restore those value after Escape is entered.

```rust
// editor.rs
    fn find(&mut self) {
        // Saving cursor position and scroll position
        let (saved_position, saved_coloff, saved_rowoff) = (self.cursor, self.coloff, self.rowoff);

        if self.prompt("Search (ESC to cancel)", Some(Editor::find_callback)).is_none() {
            self.cursor = saved_position;
            self.coloff = saved_coloff;
            self.rowoff = saved_rowoff;
        }
    }

```

We'll store the cursor position. If the search query is empty i.e Escape is pressed we will restore the cursor position. But this code won't compile successfully, as we will run through an error of Copy trait becuase of `self.cursor`. `cursor` is of type `CursorPos` which has to be updated to allow its use.

```rust
// lib.rs
#[derive(Default, Copy, Clone)]
pub struct CursorPos {
    pub x : u16,
    pub y : u16,
}

```

Now we can restore the cursor position and scroll position on pressing Escape and the cursor remains at the search query on pressing Enter.

## Search Forward and Backward ( Step 139 - 141 ) 

In our search feature we would like to allow the user to move to the next or previous occurrences of the matched word using the Arrow keys. For this, let's create another `enum SearchDirection` to have forward and backward direction for the search and initialize a variable `direction` implementing it. Also, let's store the index of the row the last matched word was in a new variable `last_match`.

```rust
// editor.rs
enum SearchDirection {
    Forward,
    Backward
}

pub struct Editor {
    /*...*/
    last_match: Option<usize>,
    direction: SearchDirection
}

fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
		/*...*/
		last_match : None;
		direction : SearchDirection::Forward
	})
}

```

We initialize `last_match` to None if there was no last matched value, and `direction` to search forward. Now, let's add code for arrow keys to move forward and backward through the matched value. So, we'll add Prev and Next to our `enum PromptKey` so that we can move to previous values on Up and Left arrow and to next values on pressing Down and Right arrow.

```rust
// editor.rs
enum PromptKey {
    /*...*/
    Prev,
    Next
}

    fn prompt(     
        &mut self,  
        pmsg: &str, 
        callback: Option<fn(&mut Editor, &str, PromptKey)>) -> Option<String> {
		let mut buf = String::from("");

        loop {
            self.set_status_msg(format!("{}: {}", pmsg, buf));
            let _ = self.refresh_screen();
	    if let Ok(c) = self.keyboard.read_key() {
		    let mut prompt_key: Option<PromptKey> = None;
		    match c { 
			    /*...*/
			    KeyEvent {
				code: KeyCode::Down,
	      			..
			    }
			    | 
			    KeyEvent {
				code: KeyCode::Right,
	      			..
			    } => {
				    if let Some(callback) = callback {
					    callback(self, &buf, PromptKey::Next);
				    }
			    },

	      	            KeyEvent {
				code: KeyCode::Up,
      				..
	      		    }
			    |
			    KeyEvent {
				code: KeyCode::Left,
      				..
			    } => {
				    if let Some(callback) = callback {
					    callback(self, &buf, PromptKey::Prev);
				    }
			    },
			    
			    _=> {}
		 }
      		 /*...*/
	     }
        }
    }

```

Let's update our `find_callback()` function to take us to the matched words. 

```rust
// editor.rs
    fn find_callback(&mut self, query: &str, event: PromptKey) {
        // To enable forward and backward search
        match event {
            PromptKey::Enter | PromptKey::Escape => {
                self.last_match = None;
                self.direction = SearchDirection::Forward;
            }

            PromptKey::Next => self.direction = SearchDirection::Forward,
            PromptKey::Prev => self.direction = SearchDirection::Backward,
            _=> {
                self.last_match = None;
                self.direction = SearchDirection::Forward;
            }
        }

        let mut current = if let Some(line) = self.last_match {
            line
        } else {
            self.direction = SearchDirection::Forward;
            self.rows.len()
        };

        for _ in 0..self.rows.len() {
            match self.direction {
                SearchDirection::Forward => {
                    current += 1;
                    if current >= self.rows.len() {
                        current = 0;
                    }
                },
                
                SearchDirection::Backward => {
                    if current == 0 {
                        current = self.rows.len() - 1;
                    } else {
                        current -= 1;
                    }
                }
            }

            if let Some(m) = self.rows[current].characters
                .match_indices(query)
                .take(1)
                .next() 
            {   
                self.last_match = Some(current);
                self.cursor.y = current as u16;
                self.cursor.x = m.0 as u16;
                self.rowoff = self.rows.len() as u16;
                break;
            }
        }
    }

```

We reset the `last_match` and `direction` to their initialized values, to be ready for the next match when the user pressed Enter or Escape. We assign movements to `Next` and `Prev`, so that the cursor reaches the matched value on pressing arrow keys.  

We create a new variable `current` to store the index of the row we are currently searching. If there is a last match it holds that value else reaches to the end of the file. We loop through the entire file, and if the `direction` is Forward then we increment the current to be at new position we are on and in case the value of current reaches the end of the file, we continue from the top; thus allowing the search to wrap around. If the `direction` is Backward then we decrement current, but before that we check if current is at top, if so then we continue from the end of the file.

Also, replace `row` with `self.rows[current]` i.e the row we are at currently. Now, let's just display the message to let the user know about forward and backward search.

```rust
// editor.rs
    fn find(&mut self) {
    	/*...*/                      
        if self.prompt("Search (Use Arrow/Enter/Esc)", Some(Editor::find_callback)).is_none() {
            self.cursor = saved_position;
            self.coloff = saved_coloff;
            self.rowoff = saved_rowoff;
        }
    }

```

And here we are done with implementing the Search feature to make our editor more interactive and user friendly!!
