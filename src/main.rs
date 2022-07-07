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
        id: organisms.0.len(),
        color: rgb::RGB {
            r: 255,
            g: 75,
            b: 50,
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
        area: window::AREA {
            upper_left: position::POSITION {
                x: 1,
                y: 1,
            },
            lower_right: position::POSITION {
                x: termion::terminal_size().unwrap().0,
                y: termion::terminal_size().unwrap().1,
            }
        },
        simulation_frame: window::Frame {
            content: window::FrameContent::Simulation,
            width_percent: 50,
            height_percent: 100,
            start_x_percent: 0,
            start_y_percent: 0,
            area: window::init_area(),
            usable_area: window::init_area(),
        },
        chart_frame: window::Frame {
            content: window::FrameContent::Chart,
            width_percent: 50,
            height_percent: 50,
            start_x_percent: 50,
            start_y_percent: 50,
            area: window::init_area(),
            usable_area: window::init_area(),
        },
    };

    organisms.0[0].spawn_organisms(25,
        window.simulation_frame.area,
        2.4,
        5.7);

    organisms.0[0].color = rgb::RGB {
        r: 0,
        g: 255,
        b: 0,
    };

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

        // RESIZED
        if window.area.to_width_height() != termion::terminal_size().unwrap() {
        //if true {
            window.update_size(window::AREA {
                upper_left: position::POSITION {
                    x: 0,
                    y: 0,
                },
                lower_right: position::POSITION {
                    x: termion::terminal_size().unwrap().0,
                    y: termion::terminal_size().unwrap().1,
                },
            });
            window.simulation_frame.resize(window);
            write!(stdout, "{}", clear::All).unwrap();
            organisms.redistribute(window.simulation_frame.usable_area);
        }

        // RENDERING
        for species in &organisms.0 {
            for organism in &species.organisms {
                organism.render(&mut stdout, organisms.clone());
            }
        }

        thread::sleep(time::Duration::from_millis(10));
    }

    // After exiting, reset terminal
    write!(stdout, "{}{}{}{}",
        cursor::Show, clear::All,
        cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Reset)).unwrap();
    //write!(stdout, "{:?}", organisms).unwrap();
    write!(stdout, "{:?}", window.area).unwrap();
    write!(stdout, "{:?}", window.simulation_frame.usable_area).unwrap();
    //write!(stdout, "{:?}{:?}", window.area.to_width_height(), termion::terminal_size().unwrap());
}
