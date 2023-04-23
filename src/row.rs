use std::cmp::min;

pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.string.len());
        let start = min(start, end);
        self.string.get(start..end).unwrap().to_string()
    }
}
