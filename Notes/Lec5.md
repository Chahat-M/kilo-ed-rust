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
                    modifiers : _
                } => self.editor_insert_char(key),
		/*...*/
	   }
	}
     }

```

