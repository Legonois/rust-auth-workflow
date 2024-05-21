use std::{char, env};
// use std::io;
// use std::io::Read;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::cursor::{self, DisableBlinking, EnableBlinking};
use crossterm::ExecutableCommand;
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
    println!("Press any key to continue...");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    println!("Continuing...")
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}

struct TerminalCleanUp;

impl Drop for TerminalCleanUp {
    // Gets called when the instance goes out of scope
    // In this implementation, when main is out of scope
    fn drop(&mut self) {
        println!("Uh oh! The tool crashed!");
        wait_until_key_pressed();
        terminal_defaults();
    }
}

fn exit() {
    // exit the program
    terminal_defaults();
    std::process::exit(0);
}

fn main() {
    let _clean_up = TerminalCleanUp;
    terminal_preferences();

    wait_until_key_pressed();


    exit();
    // let password = menu::get_password();

    // println!("You entered: {}", password);

    // let help: String = menu::help_menu();
    // println!("{}", help);

    // menu::initialize_tui();
}
