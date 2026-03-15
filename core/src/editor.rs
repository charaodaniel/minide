pub struct Editor {
    pub buffer: String,
}

impl Editor {

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

}
