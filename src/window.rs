// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use crate::position;
use std::io::Write;

pub struct Window {
    pub width: u16,
    pub height: u16,
    pub simulation_frame: Frame,
    pub chart_frame: Frame,
}

impl Window {
    pub fn new_size(&self, size: (u16, u16)) {
        self.width = size.0;
        self.height = size.1;
        self.simulation_frame.resize(self);
        self.chart_frame.resize(self);
    }
}

pub enum FrameContent {
    Simulation,
    Chart,
}

pub struct AREA {
    pub upper_left: position::POSITION,
    pub lower_right: position::POSITION,
}

pub fn init_area() -> AREA {
    AREA {
        upper_left: position::POSITION {
            x: 1,
            y: 1,
        },
        lower_right: position::POSITION {
            x: 1,
            y: 1,
        }
    }
}

pub struct Frame {
    pub content: FrameContent,
    pub width_percent: u16,
    pub height_percent: u16,
    pub start_x_percent: u16,
    pub start_y_percent: u16,
    pub area: AREA,
}

impl Frame {
    pub fn resize(&mut self, parent: &Window) {
        self.area.upper_left.x = parent.width * self.start_x_percent / 100;
        self.area.upper_left.y = parent.height * self.start_y_percent / 100;
        self.area.lower_right.x = parent.width * self.width_percent / 100 + parent.width * self.start_x_percent / 100;
        self.area.lower_right.y = parent.height * self.height_percent / 100 + parent.width * self.start_y_percent / 100;
    }

    pub fn render_frame(&self, mut stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        write!(&mut stdout,
            "g",
            ).unwrap();
    }
}
