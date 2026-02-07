use anyhow::Result;
use std::io::{self, Write};

pub fn prompt(label: &str) -> Result<String> {
    let mut stdout = io::stdout();
    stdout.write_all(label.as_bytes())?;
    stdout.flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
