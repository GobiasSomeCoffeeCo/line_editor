use std::{
    io::{stdout, Write},
    process::exit,
};

use crossterm::{
    event::DisableMouseCapture,
    event::Event,
    event::{read, KeyEvent},
    event::{EnableMouseCapture, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};

fn main() -> Result<()> {
    let mut buffer = String::new();
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("> "),
        ResetColor,
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print("> "))?
        .execute(ResetColor)?;

    terminal::enable_raw_mode()?;

    loop {
        match read()? {
            Event::Key(KeyEvent { code, modifiers }) => {
                match code {
                    KeyCode::Char(c) => {
                        let mut char_buffer = [0; 4];
                        let bytes = c.encode_utf8(&mut char_buffer).as_bytes();
                        stdout().write_all(&bytes)?;
                        stdout().flush()?;
                        buffer.push(c);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                };
            }
            Event::Mouse(event) => {
                println!("{:#?}", event)
            }
            Event::Resize(width, height) => {
                println!("width: {} and height: {}", width, height)
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
