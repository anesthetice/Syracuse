use super::syrdate::SyrDate;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Blocs(pub BTreeMap<SyrDate, f64>);

impl std::ops::Deref for Blocs {
    type Target = BTreeMap<SyrDate, f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn seconds_to_pretty_string(seconds: f64) -> String {
    let total_cs = (seconds * 100.0) as u64;

    let hours = total_cs / 360_000;
    let minutes = (total_cs % 360_000) / 6_000;
    let seconds = (total_cs % 6_000) / 100;
    let centis = total_cs % 100;

    format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hours, minutes, seconds, centis)
}

impl std::fmt::Display for Blocs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.iter()
                .sorted_by(|(a, _), (b, _)| { a.cmp(b) })
                .enumerate()
                .fold(String::new(), |acc, (idx, (date, duration))| {
                    if self.len() != idx + 1 {
                        acc + &date.to_string() + ": " + &seconds_to_pretty_string(*duration) + ", "
                    } else {
                        acc + &date.to_string() + ": " + &seconds_to_pretty_string(*duration)
                    }
                })
        )
    }
}
