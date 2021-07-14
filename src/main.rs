use std::{
    io::{stdout, Write},
    usize,
};

use crossterm::{
    cursor::{position, MoveLeft, MoveRight, MoveToColumn},
    event::Event,
    event::KeyCode,
    event::{read, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ScrollUp},
    ExecutableCommand, QueueableCommand, Result,
};
use std::io::Stdout;

fn print_message(stdout: &mut Stdout, msg: &str) -> Result<()> {
    stdout
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?
        .queue(Print(msg))?
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut buffer = String::new();
    let mut caret_pos;
    // using the macro
    // execute!(
    //     stdout(),
    //     SetForegroundColor(Color::Blue),
    //     Print("> "),
    //     ResetColor,
    // )?;

    // or using functions

    terminal::enable_raw_mode()?;
    'repl: loop {
        // print our promt
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print("> "))?
            .execute(ResetColor)?;

        // set where the input begins
        let (mut input_start_column, _) = position()?;
        input_start_column += 1;
        caret_pos = input_start_column;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            // let mut char_buffer = [0; 4];
                            // let bytes = c.encode_utf8(&mut char_buffer).as_bytes();
                            // stdout.write_all(&bytes)?;
                            // stdout.flush()?;
                            let insertion_point = caret_pos as usize - input_start_column as usize;
                            if insertion_point == buffer.len() {
                                stdout.queue(Print(c))?;
                            } else {
                                stdout
                                    .queue(Print(c))?
                                    .queue(Print(&buffer[insertion_point..]))?
                                    .queue(MoveToColumn(caret_pos + 1))?;
                            }

                            stdout.flush()?;
                            caret_pos += 1;
                            buffer.insert(insertion_point, c);
                            
                        }
                        KeyCode::Backspace => {
                            let insertion_point = caret_pos as usize - input_start_column as usize;
                            if insertion_point == buffer.len() && !buffer.is_empty() {
                                buffer.pop();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(' '))?
                                    .queue(MoveLeft(1))?;
                                stdout.flush()?;
                                caret_pos -= 1;
                            } else if insertion_point < buffer.len() && !buffer.is_empty() {
                                buffer.remove(insertion_point - 1);
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(&buffer[(insertion_point - 1)..]))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(caret_pos - 1))?;
                                stdout.flush()?;
                                caret_pos -= 1;
                            }
                        }
                        KeyCode::Delete => {
                            let insertion_point = caret_pos as usize - input_start_column as usize;
                            if insertion_point < buffer.len() && !buffer.is_empty() {
                                buffer.remove(insertion_point);
                                stdout
                                    .queue(Print(&buffer[insertion_point..]))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(caret_pos))?;
                                stdout.flush()?;
                            }
                        }
                        KeyCode::Enter => {
                            if buffer == "exit" {
                                break 'repl;
                            } else {
                                print_message(&mut stdout, &format!("Our buffer: {}", buffer))?;
                                buffer.clear();
                                break 'input;
                            }
                        }
                        KeyCode::Left => {
                            if caret_pos > input_start_column {
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                                caret_pos -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if (caret_pos as usize) < ((input_start_column as usize) + buffer.len())
                            {
                                stdout.queue(MoveRight(1))?;
                                stdout.flush()?;
                                caret_pos += 1;
                            }
                        }
                        _ => {}
                    };
                }
                Event::Mouse(event) => {
                    // println!("{:#?}", event)
                    print_message(&mut stdout, &format!("{:#?}", event))?;
                }
                Event::Resize(width, height) => {
                    print_message(
                        &mut stdout,
                        &format!("width: {} and height: {}", width, height),
                    )?;
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    println!();
    Ok(())
}