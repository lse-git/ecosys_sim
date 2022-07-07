// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use crate::window;
use rand::{
    thread_rng,
    Rng,
};

#[derive(Clone, Copy, Debug)]
pub struct POSITION {
    pub x: u16,
    pub y: u16,
}

pub fn new_rnd_pos(size: window::AREA) -> POSITION {
    let mut rng = thread_rng();
    POSITION {
        x: rng.gen_range((size.upper_left.x)..(size.lower_right.x)),
        y: rng.gen_range((size.upper_left.y)..(size.lower_right.y)),
    }
}
