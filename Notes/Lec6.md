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
