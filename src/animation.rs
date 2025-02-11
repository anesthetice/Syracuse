use std::io::{self, Write};

pub type AnimationBuilder = Vec<(String, String)>;

#[derive(Debug)]
pub struct Animation {
    index: usize,
    frames: Vec<(String, String)>,
}

impl Animation {
    pub fn construct(
        builder: AnimationBuilder,
        max_focus_len: usize,
        min_focus_len: usize,
    ) -> Self {
        let max_frame_len = builder
            .iter()
            .map(|(l, r)| l.len() + r.len())
            .max()
            .unwrap_or(0_usize)
            + max_focus_len;

        Self {
            index: 0,
            frames: builder
                .into_iter()
                .map(|(l, r)| {
                    let num_repeats = max_frame_len - min_focus_len - l.len() - r.len();
                    (
                        "\r".to_string() + l.as_str(),
                        r + " ".repeat(num_repeats).as_str(),
                    )
                })
                .collect(),
        }
    }
    pub fn step(&mut self, stdout: &mut io::Stdout, focus: &str) {
        if let Some((l, r)) = self.frames.get(self.index) {
            let _ = stdout
                .write_all(&[l.as_bytes(), focus.as_bytes(), r.as_bytes()].concat())
                .map_err(|err| {
                    eprintln!("Warning: Failed to write animation to stdout, {}", err);
                });
            let _ = stdout.flush().map_err(|err| {
                eprintln!("Warning: Failed to flush stdout, {}", err);
            });
            if self.index < self.frames.len() - 1 {
                self.index += 1
            } else {
                self.index = 0
            }
        }
    }
}
