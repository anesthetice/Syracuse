use crossterm::style::Stylize;
use std::io::Write;

use crate::warn;

pub struct SimpleAnimation;

impl Animation for SimpleAnimation {
    const FRAMES: usize = 8;
    fn play<T>(stdout: &mut std::io::Stdout, frame: &mut usize, focus: &T)
    where
        T: std::fmt::Display,
    {
        if *frame >= Self::FRAMES {
            *frame = 0;
        }
        let frame_to_draw = match *frame {
            0 => format!("\r|  {}  |      ", focus),
            1 => format!("\r|  {}  |      ", focus),
            2 => format!("\r/  {}  /      ", focus),
            3 => format!("\r/  {}  /      ", focus),
            4 => format!("\r-  {}  -      ", focus),
            5 => format!("\r-  {}  -      ", focus),
            6 => format!("\r\\  {}  \\      ", focus),
            7 => format!("\r\\  {}  \\      ", focus),
            _ => unreachable!(),
        };
        let _ = stdout
            .write_all(frame_to_draw.as_bytes())
            .map_err(|err| warn!("animation issue\n{err}"));
        let _ = stdout
            .flush()
            .map_err(|err| warn!("failed to flush to stdout\n{err}"));
        *frame += 1;
    }
}

pub trait Animation {
    const FRAMES: usize;
    fn play<T>(stdout: &mut std::io::Stdout, frame: &mut usize, focus: &T)
    where
        T: std::fmt::Display;
}
