use std::fs;
use std::io::{self, BufRead, Write};

use simape::Console;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("--engine-trace") {
        print!("{}", simape::engine_trace_fixture());
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--save-open-trace") {
        let trace = simape::save_open_continuity_fixture()
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
        print!("{trace}");
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--native-raw-summary") {
        let scenario = args.get(2).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing native raw scenario")
        })?;
        let path = args.get(3).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing native raw input path")
        })?;
        let bytes = fs::read(path)?;
        let summary = simape::native_raw_summary_from_bytes(scenario, &bytes)
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
        print!("{summary}");
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--native-raw-roundtrip") {
        let input = args.get(2).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing native raw input path")
        })?;
        let output = args.get(3).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "missing native raw output path",
            )
        })?;
        let bytes = fs::read(input)?;
        let roundtrip = simape::native_raw_roundtrip_bytes(&bytes)
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
        fs::write(output, roundtrip)?;
        return Ok(());
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut console = Console::default();

    stdout.write_all(Console::startup_text().as_bytes())?;
    stdout.flush()?;

    let mut exited_by_command = false;
    let mut commands_read = 0usize;
    for line in stdin.lock().lines() {
        let line = line?;
        commands_read += 1;
        let (response, should_quit) = console.execute_line(&line);
        stdout.write_all(response.as_bytes())?;
        stdout.flush()?;
        if should_quit {
            exited_by_command = true;
            break;
        }
    }

    if !exited_by_command {
        stdout.write_all(b"ERROR: Console failure @ ./sim/console.c 220\n")?;
        if commands_read == 0 {
            stdout.write_all(b"ERROR: Console failure @ ./sim/console.c 220\n")?;
        }
        stdout.flush()?;
    }

    Ok(())
}
