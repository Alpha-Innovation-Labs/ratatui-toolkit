use anyhow::Result;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn update(&mut self) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let mut buf = [0u8; 8192];
            let mut reader = terminal.reader.lock().unwrap();

            match reader.read(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    let mut parser = terminal.parser.lock().unwrap();
                    parser.process(&buf[..n]);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}
