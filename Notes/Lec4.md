## Moving left at the start of a line ( Step 78 )

Let's allow the user to press the left arrow in a line to reach the end of the previous line.

```
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

```
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

```
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

```
// editor.rs
KeyCode::End =>
	if self.cursor.y < self.rows.len() as u16 {
        	self.cursor.x = self.rows[self.cursor.y as usize].len() as u16; },

```
