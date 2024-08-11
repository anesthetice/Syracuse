use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::syrdate::SyrDate;

// u128 representing nanoseconds
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Blocs(BTreeMap<SyrDate, f64>);

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

pub fn sec_to_pretty_string(mut secs: f64) -> String {
    let sfloor = secs.floor();
    secs -= sfloor;
    let mut fsecs = sfloor as u32;

    let hours = fsecs / 3600;
    fsecs -= hours * 3600;
    let mins = fsecs / 60;
    fsecs -= mins * 60;

    let milis = (secs * 100.0).floor() as u32;

    format!("{:0>2}:{:0>2}:{:0>2}.{:0>3}", hours, mins, fsecs, milis)
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
                        acc + &date.to_string() + ": " + &sec_to_pretty_string(*duration) + ", "
                    } else {
                        acc + &date.to_string() + ": " + &sec_to_pretty_string(*duration)
                    }
                })
        )
    }
}

impl Blocs {
    pub fn prune(&mut self, cutoff_date: &SyrDate) -> usize {
        let _tmp = self.len();
        self.retain(|key, _| key >= cutoff_date);
        _tmp - self.len()
    }
}
