use std::io::{self, BufRead, Write};

use simape::Console;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut console = Console::default();

    stdout.write_all(Console::startup_text().as_bytes())?;
    stdout.flush()?;

    for line in stdin.lock().lines() {
        let line = line?;
        let (response, should_quit) = console.execute_line(&line);
        stdout.write_all(response.as_bytes())?;
        stdout.flush()?;
        if should_quit {
            break;
        }
    }

    Ok(())
}
