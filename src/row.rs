const KILO_TAB_STOP: usize = 8;

pub struct Row {
    characters: String,
    pub render: String
}

impl Row {

    pub fn new(characters: String) -> Self {
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

        Self{characters, render}

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
}


