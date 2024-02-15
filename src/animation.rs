use crossterm::style::{Color, Stylize};
use std::io::Write;
use crate::warn;
pub struct SimpleAnimation;

impl Animation for SimpleAnimation {
    const FRAMES: usize = 8;
    fn play<T>(stdout: &mut std::io::Stdout, frame: &mut usize, focus: &T, color: Option<&Color>)
    where T: std::fmt::Display
    {
        if *frame >= Self::FRAMES {
            *frame = 0;    
        }
        let frame_to_draw = match *frame {
            0 => format!("\r|  {:.3}  |      ", focus),
            1 => format!("\r|  {:.3}  |      ", focus),
            2 => format!("\r/  {:.3}  /      ", focus),
            3 => format!("\r/  {:.3}  /      ", focus),
            4 => format!("\r-  {:.3}  -      ", focus),
            5 => format!("\r-  {:.3}  -      ", focus),
            6 => format!("\r\\  {:.3}  \\      ", focus),
            7 => format!("\r\\  {:.3}  \\      ", focus),
            _ => unreachable!()
        };
        if let Some(color) = color {
            let _ = stdout.write_all(frame_to_draw.with(*color).to_string().as_bytes()).map_err(|err| {warn!("animation issue\n{err}")});
        } else {
            let _ = stdout.write_all(frame_to_draw.as_bytes()).map_err(|err| {warn!("animation issue\n{err}")});
        }
        *frame += 1;
    }
}

pub trait Animation {
    const FRAMES: usize;
    fn play<T>(stdout: &mut std::io::Stdout, frame: &mut usize, focus: &T, color: Option<&Color>)
    where T: std::fmt::Display;
}