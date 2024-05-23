// use crossterm::cursor;
use std::io;

// use std::{char, env};
// use std::io;
// use std::io::Read;
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, style};
use crossterm::ExecutableCommand;
// use std::io::{self, stdout, Read, Write};

#[derive(Copy, Clone)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn new(x: u16, y: u16) -> Position {
        return Position { x, y };
    }

    fn zeros() -> Position {
        return Position { x: 0, y: 0 };
    }
}


enum CursorStyle {
    Blinking,
    Solid
}

struct Cursor {
    position: Position,
    style: CursorStyle
}

impl Cursor {

    // Constructors
    fn new() -> Cursor {
        return Cursor { position: Position::zeros(), style: CursorStyle::Solid };
    }

    // Setters
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
        io::stdout().execute(cursor::MoveTo(position.x, position.y)).unwrap();
        self.position = position;
    } 

    // Utils
    pub fn draw(&self, string: &str) {
        print!("{}", string);
    }
}

pub struct DisplayController {
    cursor: Cursor,
    size: (u16, u16),
    header: String,
    footer: String,
    content: Vec<String>,
}

impl DisplayController {
    
    // Constructors
    pub fn new(header: &str, footer: &str) -> DisplayController {

        let (width, height) = terminal::size().unwrap();

        let header = header.to_string();
        let footer = footer.to_string();

        DisplayController {
            cursor: Cursor::new(),
            size: (width, height),
            header,
            footer,
            content: Vec::new(),
        }
    }

    // Setters
    fn set_size(&mut self) {
        let (width, height) = terminal::size().unwrap();

        self.size = (width, height);
    }

    fn set_style(&mut self, style: CursorStyle) {
        self.cursor.set_style(style);
    }

    pub fn set_content(&mut self, content: &str) {
        for line in content.lines() {
            self.content.push(line.to_string());
        }
    }

    // Utils
    pub fn clear_screen(&self) {
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    pub fn enable_alternate_screen(&self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        io::stdout().execute(terminal::EnterAlternateScreen).unwrap();
    }

    pub fn disable_alternate_screen(&self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        io::stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    }

    pub fn cleanup(&mut self) {
        self.disable_alternate_screen();
        self.set_style(CursorStyle::Solid);
    }

    fn render_header(&mut self) {

        if self.header.len() == 0 {
            return;
        }

        let current_position = self.cursor.position;

        self.cursor.set_position(Position::zeros());

        // println!("{}{}", style::SetBackgroundColor(style::Color::Blue), style::SetForegroundColor(style::Color::White));

        self.cursor.draw(&self.header);

        self.cursor.set_position(Position::new(0, 1));
        self.cursor.draw(&"=".repeat(self.size.0 as usize));

        self.cursor.set_position(current_position);
    }

    fn render_footer(&mut self) {
        if self.footer.len() == 0 {
            return;
        }

        let current_position = self.cursor.position;

        self.cursor.set_position(Position::new(0, self.size.1));

        // println!("{}{}", style::SetBackgroundColor(style::Color::Blue), style::SetForegroundColor(style::Color::White));

        self.cursor.draw(&self.footer);

        self.cursor.set_position(Position::new(0, self.size.1 - 2));
        self.cursor.draw(&"=".repeat(self.size.0 as usize));

        self.cursor.set_position(current_position);
    }

    fn render_content(&mut self) {
        if self.content.is_empty() {
            return;
        }

        let current_position = self.cursor.position;
        let start_y = 2;
        let end_y = self.size.1 - 2;

        self.cursor.set_position(Position::new(0, start_y));

        for line in &self.content {
            for char in line.chars() {
                if self.cursor.position.y >= end_y {
                    break;
                }

                if self.cursor.position.x >= self.size.0 {
                    self.cursor.set_position(Position::new(0, self.cursor.position.y + 1));
                }

                if self.cursor.position.y < end_y {
                    self.cursor.draw(&char.to_string());
                    self.cursor.set_position(Position::new(self.cursor.position.x + 1, self.cursor.position.y));
                }
            }

            if self.cursor.position.y < end_y {
                self.cursor.set_position(Position::new(0, self.cursor.position.y + 1));
            } else {
                break;
            }
        }

        self.cursor.set_position(current_position);
    }

    pub fn render(&mut self) {
        self.set_size();
        self.clear_screen();
        self.render_header();
        self.render_content();
        self.render_footer();
    }

    // set text
}

impl Drop for DisplayController {
    fn drop(&mut self) {
        self.cleanup();
    }
}


