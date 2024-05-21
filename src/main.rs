use std::{char, env};
// use std::io;
// use std::io::Read;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::cursor::{self, DisableBlinking, EnableBlinking};
use crossterm::ExecutableCommand;
use std::io::{self, Write, Read};

// modules
mod menu;

// exit codes:
// 1 | Keyboard Interrupt

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

    // for (index, arg) in args.iter().enumerate() {
    //     println!("Arguement[{}]: {}", index, arg);
    // }
}

struct TerminalCleanUp;

impl Drop for TerminalCleanUp {
    // Gets called when the instance goes out of scope
    // In this implementation, when main is out of scope
    fn drop(&mut self) {
        println!("wiah");
        terminal_defaults();
    }
}

fn main() {
    let _clean_up = TerminalCleanUp;
    terminal_preferences();
    println!("Enter Password");
    wait_until_key_pressed();

    let mut input = String::new();

    // let mut interation: u32 = 0;

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Event::Key(event) = event::read().expect("Failed to read line") {

                // Only account for key presses
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                let mut char_input: Option<char> = None;

                match event.code {
                    // Exit if control + C is pressed
                    KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' => {
                        panic!("Keyboard Interrupt | Exiting");
                    }
    
                    KeyCode::Enter => {
                        break;
                    }
    
                    // Output inputted char
                    KeyCode::Char(c) => {
                        char_input = Some(c);
                    }

                    _ => {
                        // continue;
                    }
                }

                if let Some(char) = char_input {
                    // interation += 1;

                    print!("{}\n", char);
                    input.push(char);
                }

            }
        }
    }

    // loop {
    //     io::stdout().flush().unwrap();

    //     if event::poll(std::time::Duration::from_millis(500)).unwrap() {

    //         if let Event::Key(key_event) = event::read().unwrap() {

    //             println!("{}", iterations);

    //             match key_event.code {
    //                KeyCode::Char(c) if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' => {
    //                    panic!("Keyboard Interrupt | Exiting");
    //                }
    //                 KeyCode::Char(c) => {
    //                     print!("{}", c);
    //                     io::stdout().flush().unwrap();
    //                     input.push(c);
    //                 }
    //                 KeyCode::Enter => {
    //                     break;
    //                 }
    //                 KeyCode::F(2) => {
    //                     break;
    //                 }
    //                 // Default case
    //                 _ => {} // ignore other keys
    //             }

    //             iterations += 1;
    //         }
    //     }
    // }

    println!("You entered: {}", input);

    let help: String = menu::help_menu();
    println!("{}", help);

    wait_until_key_pressed();
}
