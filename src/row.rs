use std::cmp::min;

use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len_v2();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.string.len());
        let start = min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ")
            } else {
                result.push_str(grapheme)
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }
    pub fn update_len_v2(&mut self) {
        self.len = self
            .string
            .char_indices()
            .filter(|&(_, c)| c != '\t')
            .count()
    }
}
