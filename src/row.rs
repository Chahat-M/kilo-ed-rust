const KILO_TAB_STOP: usize = 8;

pub struct Row {
    chars: String,
    pub render: String
}

impl Row {

    pub fn new(chars: String) -> Self {
        let mut render = String::new();
        let mut idx = 0;

        for c in chars.chars(){
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

        Self{chars, render}

    }

    pub fn render_length(&self) -> usize {
        self.render.len()
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

}


