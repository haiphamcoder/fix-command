use crossterm::{cursor, event, execute, style, terminal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::style::Stylize;
use std::io::{stdout, Write};

pub fn select_suggestion(suggestions: &[String]) -> anyhow::Result<Option<String>> {
    if suggestions.is_empty() { return Ok(None); }
    let mut idx: usize = 0;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;
    loop {
        draw(suggestions, idx)?;
        if let Event::Key(k) = event::read()? {
            if k.kind == KeyEventKind::Press {
                match k.code {
                    KeyCode::Up => { if idx > 0 { idx -= 1; } }
                    KeyCode::Down => { if idx + 1 < suggestions.len() { idx += 1; } }
                    KeyCode::Enter => { cleanup()?; return Ok(Some(suggestions[idx].clone())); }
                    KeyCode::Esc | KeyCode::Char('c') if k.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        cleanup()?; return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw(suggestions: &[String], idx: usize) -> anyhow::Result<()> {
    let mut out = stdout();
    execute!(out, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0))?;
    println!("Use ↑/↓ then Enter. Ctrl+C to cancel.\n");
    for (i, s) in suggestions.iter().enumerate() {
        if i == idx {
            println!("{} {}", style::style("→").attribute(style::Attribute::Bold), s);
        } else {
            println!("  {}", s);
        }
    }
    out.flush()?;
    Ok(())
}

fn cleanup() -> anyhow::Result<()> {
    let mut out = stdout();
    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

