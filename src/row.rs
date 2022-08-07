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
}


