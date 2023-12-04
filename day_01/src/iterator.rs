pub struct StringNumbers {
    text: String,
    index: usize,
}

impl StringNumbers {
    pub fn new(text: String) -> StringNumbers {
        StringNumbers {
            text,
            index: 0,
        }
    }

    fn match_substring(&self, substr: &str) -> Option<u32> {
        [
            (["one", "1"], 1),
            (["two", "2"], 2),
            (["three", "3"], 3),
            (["four", "4"], 4),
            (["five", "5"], 5),
            (["six", "6"], 6),
            (["seven", "7"], 7),
            (["eight", "8"], 8),
            (["nine", "9"], 9),
        ]
            .iter()
            .find(|(sub, _)| starts_with_any(sub.to_vec(), substr))
            .map(|(_, num)| *num)
    }
}

impl Iterator for StringNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            if self.text.len() <= self.index {
                return None
            }

            let slice = &self.text[self.index..];
            let num = self.match_substring(slice);

            self.index += 1;

            if num.is_none() {
                continue;
            }

            return num;
        }
    }
}

fn starts_with_any(substrs: Vec<&str>, text: &str) -> bool {
    substrs.iter().any(|sub| text.starts_with(sub))
}
