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

    //TEMP
    organisms.0.push(organisms::Species {
        name: String::from("Mole"),
        dominance: 5,
        color: rgb::RGB {
            r: 0,
            g: 50,
            b: 200,
        },
        repr_char: '@',
        behav_to_stronger: organisms::Behavior::Passive,
        behav_to_equal: organisms::Behavior::Aggressive {
            success_probability: 0.5
        },
        behav_to_weaker: organisms::Behavior::Aggressive {
            success_probability: 0.9
        },
        organisms: vec![],
        population_size: 0,
    });

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
        chart_frame: window::Frame {
            content: window::FrameContent::Chart,
            width_percent: 50,
            height_percent: 50,
            start_x_percent: 50,
            start_y_percent: 50,
            area: window::init_area(),
        },
    };

    organisms.0[0].clone().spawn_organisms(15,
        window.simulation_frame.area,
        5.7);

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
            write!(stdout, "{}", clear::All).unwrap();
            organisms.redistribute(window.simulation_frame.area);
        }

        // RENDERING
        for species in organisms.0.clone() {
            for organism in species.organisms.clone() {
                organism.render(&mut stdout);
            }
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
