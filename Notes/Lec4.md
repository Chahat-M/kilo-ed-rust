## Moving left at the start of a line

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

## Moving right at the end of the line

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
