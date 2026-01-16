use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::sync::{Arc, Mutex};
use vt100::Parser;

use crate::fuzzy_finder::{FuzzyFinder, FuzzyFinderTerminal};

impl FuzzyFinder {
    pub fn spawn_command(
        &mut self,
        cmd: &str,
        args: &[&str],
        stdin: Option<String>,
        rows: u16,
        cols: u16,
    ) -> Result<()> {
        if rows == 0 || cols == 0 {
            return Err(anyhow::anyhow!("Invalid terminal size: {}x{}", rows, cols));
        }

        let pty_system = native_pty_system();

        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = pty_system
            .openpty(pty_size)
            .context("Failed to allocate PTY")?;

        let mut command = CommandBuilder::new(cmd);
        for arg in args {
            command.arg(arg);
        }

        let child = pair
            .slave
            .spawn_command(command)
            .context("Failed to spawn command")?;

        #[cfg(unix)]
        {
            if let Some(fd) = pair.master.as_raw_fd() {
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
            }
        }

        let reader = pair.master.try_clone_reader()?;
        let mut writer = pair.master.take_writer()?;

        if let Some(input) = stdin {
            writer.write_all(input.as_bytes())?;
            writer.flush()?;
        }

        let parser = Arc::new(Mutex::new(Parser::new(rows, cols, 0)));

        self.terminal = Some(FuzzyFinderTerminal {
            parser,
            _master: Arc::new(Mutex::new(pair.master)),
            child: Arc::new(Mutex::new(child)),
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        });

        Ok(())
    }
}
