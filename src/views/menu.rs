use std::{char, env};
// use std::io;
// use std::io::Read;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::cursor;
use crossterm::ExecutableCommand;
use std::io::{self, stdout, Read, Write};

mod views;

// struct Position {
//     x: u16,
//     y: u16,
// }

// impl Position {
//     fn new(x: u16, y: u16) -> Position {
//         return Position { x, y };
//     }

//     fn zeros() -> Position {
//         return Position { x: 0, y: 0 };
//     }
// }

enum CursorStyle {
    Blinking,
    Solid
}

struct Cursor {
    position: Position,
    style: CursorStyle
}

impl Cursor {

    fn new() -> Cursor {
        return Cursor { position: Position::zeros(), style: CursorStyle::Solid };
    }

    fn set_style(&mut self, style: CursorStyle) {

        match style {
            CursorStyle::Blinking => {
                io::stdout().execute(cursor::EnableBlinking).unwrap();
            }
            CursorStyle::Solid => {
                io::stdout().execute(cursor::DisableBlinking).unwrap();
            }
        }

        self.style = style;
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    } 
}

struct Terminal {
    cursor: Cursor,
    size: (u16, u16),
}

impl Terminal {
    fn new() -> Terminal {
        let terminal =  Terminal {
            cursor: Cursor::new(),
            size: (0, 0)
        };

        return terminal;
    }

    fn set_size(&mut self) {
        let window_size = terminal::window_size().expect("Error finding window size");
        self.size = (window_size.width, window_size.height)
    }
}

pub fn initialize_tui() {
    Terminal::new();
}


pub fn help_menu() -> String {

    let help_menu = String::from(
        "Usage: 
        ./main.exe <search> <filename>
        
        Search for a pattern in a file and display the lines that contain it.
        "
    );

    help_menu
}

pub fn version_menu() -> String {

    let version_menu = String::from(
        "Version: 0.1.0"
    );

    version_menu
}

pub fn about_menu() -> String {

    let about_menu = String::from(
        "About: 
        This is a simple program that hashes and salts passwords.
        "
    );

    about_menu
}

fn get_text_consume_input() -> String {
    return get_text(true);
}

fn get_text(consume_input: bool) -> String {

    let mut output = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {

            if let Event::Key(event) = event::read().expect("Failed to read line") {


                // Only account for key presses
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                match event.code {
                    // Exit if control + C is pressed
                    KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' => {
                        panic!("Keyboard Interrupt | Exiting");
                    }
    
                    KeyCode::Enter => {
                        print_char('\n');
                        break;
                    }
    
                    // Output inputted char
                    KeyCode::Char(c) => {
                        if !consume_input {
                            print_char(c);
                        }
                        output.push(c);
                    }

                    KeyCode::Backspace => {
                        if !consume_input {
                            remove_char();
                        }
                        output.pop();
                    }

                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    return output;
}

fn get_cursor_position() -> Position {
    let position: Position;

    match cursor::position() {
        Ok((x, y)) => {
            position = Position::new(x, y);
            // println!("The current cursor position is: ({}, {})", x, y);
        }
        Err(e) => {
            panic!("Failed to get the cursor position: {}", e);
        }
    }

    return position;
}

fn move_cursor(position: Position) {
    stdout().execute(cursor::MoveTo(position.x, position.y)).unwrap();
}

fn print_char(c: char) {

    print!("{}", c);

    if c == '\n' {
        let mut position = get_cursor_position();
        position.x = 0;

        move_cursor(position);
    } else {
        increment_cursor();
    }
}

fn increment_cursor() {
    let mut position = get_cursor_position();

    position.x += 1;
    move_cursor(position);
}

fn decrement_cursor() {
    let mut position = get_cursor_position();

    if position.x - 1 == 0 {
        return;
    }

    position.x -= 1;
    move_cursor(position);
}

fn remove_char() {
    decrement_cursor();
    print!(" ");
    decrement_cursor();
}


pub fn get_password() -> String {
    println!("Enter Password");
    get_text(false)
}