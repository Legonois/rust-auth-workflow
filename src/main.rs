// use std::{char, env};
// use std::io;
// use std::io::Read;
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::cursor::{DisableBlinking, EnableBlinking};
use crossterm::ExecutableCommand;
use views::display_controller;
// use views::{display_controller, DisplayController};
use std::io::{self, Write, Read};

mod views;

fn terminal_preferences() {
    enable_raw_mode().expect("Failed to enable raw mode");
    io::stdout().execute(terminal::EnterAlternateScreen).unwrap();
    io::stdout().execute(EnableBlinking).unwrap();
}

fn terminal_defaults() {
    disable_raw_mode().expect("Failed to disable raw mode");
    io::stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    io::stdout().execute(DisableBlinking).unwrap();
}

fn wait_until_key_pressed() {
    enable_raw_mode().expect("Failed to enable raw mode");
    io::stdout().execute(EnableBlinking).unwrap();

    // println!("Press any key to continue...");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    disable_raw_mode().expect("Failed to disable raw mode");
    io::stdout().execute(DisableBlinking).unwrap();
}

fn main() {
    let header = "Rust Authentication Workflow";
    let footer = "Copyright 2024 - Legonois";

    let mut display_controller = display_controller::DisplayController::new(header, footer);

    display_controller.set_content("Hello world! \nThis is my new project.\n\nPlease enter your password:jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj\n");

    display_controller.enable_alternate_screen();
    display_controller.render();

    wait_until_key_pressed();
}
