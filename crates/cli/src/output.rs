use std::io::{self, Write};
use std::process::{Command, Stdio};
use termimad::crossterm::style::Color;
use termimad::MadSkin;

pub fn print_answer(answer: &str, format: bool) {
    if format {
        if render_with_external(answer) {
            return;
        }
        let skin = colored_skin();
        skin.print_text(answer);
    } else {
        println!("{answer}");
    }
}

fn render_with_external(answer: &str) -> bool {
    let preferred = std::env::var("CEALUM_MD_RENDERER").ok();
    let candidates: Vec<&str> = match preferred.as_deref() {
        Some("glow") => vec!["glow"],
        Some("mdcat") => vec!["mdcat"],
        Some(other) => vec![other],
        None => vec!["glow", "mdcat"],
    };

    let mut missing_glow = false;

    for &program in &candidates {
        match try_render(program, answer) {
            Ok(()) => {
                if missing_glow && program != "glow" {
                    eprintln!("warning: `glow` not found; install it for the best markdown colors");
                }
                return true;
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                if program == "glow" {
                    missing_glow = true;
                }
                continue;
            }
            Err(_) => continue,
        }
    }

    if missing_glow {
        eprintln!("warning: `glow` not found; install it for the best markdown colors");
    }

    if preferred.is_some() || candidates != vec!["glow", "mdcat"] {
        eprintln!(
            "markdown renderer not available; install `glow` or `mdcat` for colored output"
        );
    }
    false
}

fn try_render(program: &str, answer: &str) -> io::Result<()> {
    let mut child = Command::new(program)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(answer.as_bytes())?;
    }

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{program} exited with status {status}"),
        ))
    }
}

fn colored_skin() -> MadSkin {
    let mut skin = MadSkin::default_dark();
    skin.headers[0].set_fg(Color::Cyan);
    skin.headers[1].set_fg(Color::Blue);
    skin.headers[2].set_fg(Color::Green);
    skin.bold.set_fg(Color::Yellow);
    skin.inline_code.set_fgbg(Color::Black, Color::DarkGrey);
    skin.code_block.set_fgbg(Color::Grey, Color::DarkGrey);
    skin.quote_mark = termimad::StyledChar::from_fg_char(Color::DarkCyan, '▐');
    skin
}
