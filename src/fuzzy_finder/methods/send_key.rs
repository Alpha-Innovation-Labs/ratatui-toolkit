use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn send_key(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let key_bytes = key_event_to_bytes(key);
            let mut writer = terminal.writer.lock().unwrap();
            writer.write_all(&key_bytes)?;
            writer.flush()?;
        }
        Ok(())
    }
}

fn key_event_to_bytes(key: KeyEvent) -> Vec<u8> {
    use crossterm::event::{KeyCode, KeyModifiers};

    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match c {
                    'a'..='z' => vec![(c as u8) - b'a' + 1],
                    _ => vec![c as u8],
                }
            } else {
                c.to_string().into_bytes()
            }
        }
        KeyCode::Enter => vec![b'\r'],
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Delete => vec![0x1b, b'[', b'3', b'~'],
        KeyCode::Left => vec![0x1b, b'[', b'D'],
        KeyCode::Right => vec![0x1b, b'[', b'C'],
        KeyCode::Up => vec![0x1b, b'[', b'A'],
        KeyCode::Down => vec![0x1b, b'[', b'B'],
        KeyCode::Home => vec![0x1b, b'[', b'H'],
        KeyCode::End => vec![0x1b, b'[', b'F'],
        KeyCode::PageUp => vec![0x1b, b'[', b'5', b'~'],
        KeyCode::PageDown => vec![0x1b, b'[', b'6', b'~'],
        KeyCode::Tab => vec![b'\t'],
        KeyCode::Esc => vec![0x1b],
        _ => vec![],
    }
}
