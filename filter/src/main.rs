use std::env;
use std::io::{self, BufRead, IsTerminal, Lines, StdinLock, Write};

fn filter_line(line: &str, filter: &str) -> bool {
    line.contains(filter)
}

fn filter_lines(lines: Lines<StdinLock<'_>>, filter: &str) -> Vec<String> {
    let filtered = lines
        .filter_map(Result::ok)
        .filter(|l| filter_line(l, filter));
    filtered.collect()
}

fn paint_line(line: &str, filter: &str) -> String {
    line.replace(filter, &format!("\x1b[32;1m{}\x1b[0m", filter))
}

fn run(filter: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let lines = stdin.lock().lines();

    let filtered = filter_lines(lines, filter);
    if stdout.is_terminal() {
        filtered
            .iter()
            .try_for_each(|line| writeln!(handle, "{}", paint_line(line, filter)))?;
    } else {
        filtered
            .iter()
            .try_for_each(|line| writeln!(handle, "{}", line))?;
    }

    handle.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <word>", args[0]);
        std::process::exit(1);
    }
    run(&args[1])
}
