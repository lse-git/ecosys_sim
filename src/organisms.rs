// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use crate::position;
use crate::rgb;
use crate::window;
use std::io::Write;

#[derive(Clone)]
pub enum Behavior {
    Aggressive {
        success_probability: f32,
    },
    Defensive {
        success_probability: f32,
    },
    Passive,
}

#[derive(Clone)]
pub struct Organism {
    species: Species,
    pos: position::POSITION,
    strength: f32,
}

impl Organism {
    pub fn render(&self, mut stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        write!(&mut stdout,
            "{}{}{}",
            termion::cursor::Goto(self.pos.x, self.pos.y),
            termion::color::Fg(termion::color::Rgb(self.species.color.r, self.species.color.g, self.species.color.b)),
            self.species.repr_char,
            ).unwrap();
    }
}

#[derive(Clone)]
pub struct Species {
    name: String,
    dominance: u16,
    color: rgb::RGB,
    repr_char: char,
    behav_to_stronger: Behavior,
    behav_to_equal: Behavior,
    behav_to_weaker: Behavior,
    pub organisms: Vec<Organism>,
    population_size: u32,
}

impl Species {
    // Function to spawn and randomly distribute a bunch of organisms of same species
    pub fn spawn_organisms(mut self, num: u32, area: window::AREA, strength: f32) {
        for _i in 0..num {
            self.organisms.push(Organism {
                species: self,
                pos: position::new_rnd_pos(area),
                strength,
            });
        }
    }
}

#[derive(Clone)]
pub struct Organisms(pub Vec<Species>);

impl Organisms {
    pub fn redistribute(self, area: window::AREA) {
        for mut species in self.0 {
            for mut organism in species.organisms {
                organism.pos = position::new_rnd_pos(area);
            
            }
        }
    }
}
