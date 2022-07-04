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
mod window;

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

    // Create an object of Type Window
    let mut window = window::Window {
        width: termion::terminal_size().unwrap().0,
        height: termion::terminal_size().unwrap().1,
        simulation_frame: window::Frame {
            content: window::FrameContent::Simulation,
            width_percent: 50,
            height_percent: 100,
            start_x_percent: 0,
            start_y_percent: 0,
            area: window::init_area(),
        },
        settings_frame: window::Frame {
            content: window::FrameContent::Settings,
            width_percent: 50,
            height_percent: 50,
            start_x_percent: 50,
            start_y_percent: 0,
            area: window::init_area(),
        },
        chart_frame: window::Frame {
            content: window::FrameContent::Chart,
            width_percent: 50,
            height_percent: 50,
            start_x_percent: 50,
            start_y_percent: 50,
            area: window::init_area(),
        },
    };

    organisms.spawn_organisms::<137>("test",
        4.3,
        rgb::RGB {r: 0, g: 200, b: 50},
        '.',
        (window.width * window.simulation_frame.width_percent / 100,
         window.height * window.simulation_frame.height_percent / 100));
    organisms.spawn_organisms::<14>("also_test",
        7.3,
        rgb::RGB {r: 255, g: 20, b: 20},
        '#',
        (window.width * window.simulation_frame.width_percent / 100,
         window.height * window.simulation_frame.height_percent / 100));

    // MAIN LOOP
    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                // Exit if esc is pressed
                Key::Esc => break,
                _ => (),
            }
        }
        if (window.width, window.height) != termion::terminal_size().unwrap() {
            window.new_size(termion::terminal_size().unwrap());
            organisms.clone().redistribute((window.width, window.height));
        }

        // RENDERING
        for organism in &organisms.0 {
            organism.render(&mut stdout);
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
