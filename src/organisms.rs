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

#[derive(Clone, Debug)]
pub enum Behavior {
    Aggressive {
        success_probability: f32,
    },
    Defensive {
        success_probability: f32,
    },
    Passive,
}

#[derive(Clone, Debug)]
pub struct Organism {
    species_id: usize,
    pos: position::POSITION,
    speed: f32,
    strength: f32,
}

impl Organism {
    pub fn species_from_id(&self, organisms_vec: &Organisms) -> Species {
        organisms_vec.0[self.species_id].clone()
    }

    pub fn render(&self, mut stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, organisms_vec: Organisms) {
        write!(&mut stdout,
            "{}{}{}",
            termion::cursor::Goto(self.pos.x, self.pos.y),
            termion::color::Fg(termion::color::Rgb(
                    self.species_from_id(&organisms_vec).color.r,
                    self.species_from_id(&organisms_vec).color.g,
                    self.species_from_id(&organisms_vec).color.b)),
            self.species_from_id(&organisms_vec).repr_char,
            ).unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct Species {
    pub name: String,
    pub id: usize,
    pub dominance: u16,
    pub color: rgb::RGB,
    pub repr_char: char,
    pub behav_to_stronger: Behavior,
    pub behav_to_equal: Behavior,
    pub behav_to_weaker: Behavior,
    pub organisms: Vec<Organism>,
    pub population_size: u32,
}

impl Species {
    // Function to spawn and randomly distribute a bunch of organisms of same species
    pub fn spawn_organisms(&mut self, num: u32, area: window::AREA, speed: f32, strength: f32) {
        for _i in 0..num {
            self.organisms.push(Organism {
                species_id: self.id,
                pos: position::new_rnd_pos(area.clone()),
                speed,
                strength,
            });
        }
        self.population_size += num
    }
}

#[derive(Clone, Debug)]
pub struct Organisms(pub Vec<Species>);

impl Organisms {
    pub fn redistribute(&mut self, area: window::AREA) {
        for species in &mut self.0 {
            for mut organism in &mut species.organisms {
                organism.pos = position::new_rnd_pos(area);
            }
        }
    }
}
