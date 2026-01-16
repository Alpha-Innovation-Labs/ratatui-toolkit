use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn get_selection(&mut self) -> Option<String> {
        if let Some(terminal) = &self.terminal {
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();
            let contents_bytes = screen.contents_formatted();
            let contents_str = String::from_utf8_lossy(&contents_bytes);

            for line in contents_str.lines().rev() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
        None
    }
}
