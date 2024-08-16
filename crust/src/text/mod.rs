pub struct SourceText {
    text: String,
}

impl SourceText {
    pub fn new(text: String) -> Self {
        Self {
            text,
        }
    }

    pub fn line_index(&self, position: usize) -> usize {
        self.text[..position].lines().count();
    }
}
