// Author:
// _____ ___
//|  ___/ _ \__/\__
//| |_ | | | \    /
//|  _|| |_| /_  _\
//|_|   \___/  \/


use crate::position;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Window {
    pub area: AREA,
    pub simulation_frame: Frame,
    pub chart_frame: Frame,
}

impl Window {
    pub fn update_size(&mut self, size: AREA) {
        self.area = size;
    }
}

#[derive(Clone, Copy)]
pub enum FrameContent {
    Simulation,
    Chart,
}

#[derive(Clone, Copy, Debug)]
pub struct AREA {
    pub upper_left: position::POSITION,
    pub lower_right: position::POSITION,
}

impl AREA {
    pub fn to_width_height(&self) -> (u16, u16) {
        (
            self.lower_right.x - self.upper_left.x,
            self.lower_right.y - self.upper_left.y,
        )
    }
}

pub fn init_area() -> AREA {
    AREA {
        upper_left: position::POSITION {
            x: 1,
            y: 1,
        },
        lower_right: position::POSITION {
            x: 2,
            y: 2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Frame {
    pub content: FrameContent,
    pub width_percent: u16,
    pub height_percent: u16,
    pub start_x_percent: u16,
    pub start_y_percent: u16,
    pub area: AREA,
    pub usable_area: AREA,
}

impl Frame {
    pub fn resize(&mut self, parent: Window) {
        let parent_width = parent.area.lower_right.x - parent.area.upper_left.x;
        let parent_height = parent.area.lower_right.y - parent.area.upper_left.y;
        self.area.upper_left.x = (parent_width as f32 * self.start_x_percent as f32 / 100.0) as u16;
        self.area.upper_left.y = (parent_height as f32 * self.start_y_percent as f32 / 100.0) as u16;
        self.area.lower_right.x = (parent_width as f32 * self.width_percent as f32 / 100.0 + parent_width as f32 * self.start_x_percent as f32 / 100.0) as u16;
        self.area.lower_right.y = (parent_height as f32 * self.height_percent as f32 / 100.0 + parent_width as f32 * self.start_y_percent as f32 / 100.0) as u16;
        self.usable_area.upper_left.x = self.area.upper_left.x + 1;
        self.usable_area.upper_left.y = self.area.upper_left.y + 1;
        self.usable_area.lower_right.x = self.area.lower_right.x - 1;
        self.usable_area.lower_right.y = self.area.lower_right.y - 1;
    }

    pub fn render_frame(&self, mut stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        write!(&mut stdout,
            "g",
            ).unwrap();
    }
}
