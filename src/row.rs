const KILO_TAB_STOP: usize = 8;

pub struct Row {
    pub characters: String,
    pub render: String
}

impl Row {

    pub fn new(characters: String) -> Self {
        let render = Row::render_row(&characters);
        Self{characters, render}
    }

    // Function to render tabs as multiple spaces and store it 
    pub fn render_row(characters: &str) -> String {
        let mut render = String::new();
        let mut idx = 0;

        for c in characters.chars(){
            match c {
                '\t' => {
                    render.push(' ');
                    idx += 1;
                    while idx % KILO_TAB_STOP != 0 {
                        render.push(' ');
                        idx += 1;
                    }
                },
                _ => {
                    render.push(c);
                    idx += 1;
                }
            }
        }
        render
    }

    pub fn render_length(&self) -> usize {
        self.render.len()
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn cursorx_to_renderx(&self, cx: u16) -> u16 {
        let mut rx = 0;

        for c in self.characters.chars().take(cx as usize) {
            if c == '\t' {
                rx += (KILO_TAB_STOP - 1) - (rx % KILO_TAB_STOP);
            }
            rx += 1;
        }
        rx as u16
    }

    /*
    pub fn renderx_to_cursorx(&self, rx: usize) -> u16 {
        let mut current_rx = 0;

        for (cx, c) in self.characters.chars().enumerate() {
            if c == '\t' {
                current_rx += (KILO_TAB_STOP - 1) - (current_rx % KILO_TAB_STOP);
            }
            current_rx += 1;

            if current_rx > rx {
                return cx as u16;
            }
        }
        self.characters.len() as u16
    }
    */

    // Function to insert a character at any position
    pub fn row_insert_char(&mut self, at: usize, c: char) {
        if at >= self.characters.len() {
            self.characters.push(c); }
        else {
            self.characters.insert(at, c);
        }
        self.render = Row::render_row(&self.characters);
    }

    // Returns true if a character is deleted else false
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

    pub fn append_string(&mut self, s: &str) {
        self.characters.push_str(s);
        self.render = Row::render_row(&self.characters);
    }
    
    // Function to split the text of a row, where the cursor is, like when we press Enter
    pub fn rowsplit(&mut self, from: usize) -> String {
        // split_off -> returns [from, len) and updates self to [0, from)
        let next_row = self.characters.split_off(from);
        self.render = Row::render_row(&self.characters);

        next_row
    }

}


