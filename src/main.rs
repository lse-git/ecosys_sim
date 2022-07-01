// Author:
//  _____ ___
// |  ___/ _ \__/\__
// | |_ | | | \    /
// |  _|| |_| /_  _\
// |_|   \___/  \/


// Initialize termion and rand crate
extern crate termion;

// Import all needed crates
use std::io::{stdout, Write};
use std::{
    time,
    thread,
};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{
    clear,
    cursor,
};

// Import modules
mod position;
mod rgb;
mod organisms;

// MAIN FUNCTION
fn main() {
    // Get input stream
    let mut stdin = termion::async_stdin().keys();
    // Enter raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Hide cursor
    write!(stdout, "{}{}{}", clear::All, cursor::Hide, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    let mut organisms = organisms::Organisms(vec![]);

    organisms.spawn_organisms::<137>("test",
        4.3,
        rgb::RGB {r: 0, g: 200, b: 50},
        '.',
        termion::terminal_size().unwrap());
    organisms.spawn_organisms::<14>("also_test",
        7.3,
        rgb::RGB {r: 255, g: 20, b: 20},
        '#',
        termion::terminal_size().unwrap());

    // MAIN LOOP
    let mut ter_size = termion::terminal_size().unwrap();
    loop {
        let old_ter_size = ter_size;
        ter_size = termion::terminal_size().unwrap();
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                // Exit if esc is pressed
                Key::Esc => break,
                _ => (),
            }
        }
        if old_ter_size != ter_size {
            todo!();
        }

        // RENDERING
        for i in &organisms.0 {
            let buffer = i.render();
            write!(stdout, "{}{}{}", buffer.0, buffer.1, buffer.2).unwrap();
        }

        thread::sleep(time::Duration::from_millis(2));
    }

    // After exiting, reset terminal
    write!(stdout, "{}{}{}{}",
        cursor::Show, clear::All,
        cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Reset)).unwrap();
    //write!(stdout, "{:?}", test).unwrap();
}
