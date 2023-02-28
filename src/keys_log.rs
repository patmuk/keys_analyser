use std::fmt;

pub struct KeysLog(Vec<(String, u32)>);

impl fmt::Display for KeysLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.0 {
            writeln!(f, "{} - {}", entry.1, entry.0)?;
        }
        Ok(())
    }
}

impl KeysLog {
    pub fn new() -> Self {
        let vec: Vec<(String, u32)> = Vec::new();
        KeysLog(vec)
    }

    pub fn log(&mut self, sequence: &str) {
        match self.0.iter().position(|entry| entry.0 == sequence) {
            Some(index) => {
                let entry = &mut self.0[index];
                let count = entry.1 + 1;
                *entry = (sequence.to_owned(), count);
            }
            None => self.0.push((sequence.to_owned(), 1)),
        }
    }

    pub fn sort(&mut self) {
        self.0.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    }
}
