// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use rand::{
    thread_rng,
    Rng,
};

#[derive(Clone, Debug)]
pub struct POSITION {
    pub x: u16,
    pub y: u16,
}

pub fn new_rnd_pos(size: (u16, u16)) -> POSITION {
    let mut rng = thread_rng();
    POSITION {
        x: rng.gen_range(0..size.0),
        y: rng.gen_range(0..size.1),
    }
}
