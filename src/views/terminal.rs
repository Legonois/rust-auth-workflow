use std::{char, env};
// use std::io;
// use std::io::Read;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::cursor;
use crossterm::ExecutableCommand;
use std::io::{self, stdout, Read, Write};

struct Position {
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

