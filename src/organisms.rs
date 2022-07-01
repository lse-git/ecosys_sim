// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use crate::position;
use crate::rgb;

enum InteractionType {
    GoodGood,
    GoodBad,
    GoodNeutral,
    NeutralBad,
    NeutralNeutral,
    BadBad,
}

#[derive(Clone, Debug)]
pub struct Organism {
    species: String,
    pos: position::POSITION,
    speed: f32,
    color: rgb::RGB,
    repr_char: char,
}

impl Organism {
    pub fn render(&self) -> (termion::cursor::Goto, termion::color::Fg<termion::color::Rgb>, char) {
        (
            termion::cursor::Goto(self.pos.x, self.pos.y),
            termion::color::Fg(termion::color::Rgb(self.color.r, self.color.g, self.color.b)),
            self.repr_char,
        )
    }
}

pub struct Organisms(pub Vec<Organism>);

impl Organisms {
    // Function to spawn and randomly distribute a bunch of organisms of same species
    pub fn spawn_organisms<const NUM: usize>(&mut self, species: &str, speed: f32, color: rgb::RGB, repr_char: char, ter_size: (u16, u16)) {
        for _i in 0..(NUM as u32) {
            self.0.push(Organism {
                species: String::from(species),
                pos: position::new_rnd_pos(ter_size),
                speed,
                color: color.clone(),
                repr_char,
            });
        }
    }
}
