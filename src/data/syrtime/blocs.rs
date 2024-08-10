use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::syrdate::SyrDate;

// u128 representing nanoseconds
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Blocs(BTreeMap<SyrDate, u128>);

impl std::ops::Deref for Blocs {
    type Target = BTreeMap<SyrDate, u128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn ns_to_pretty_string(mut nanoseconds: u128) -> String {
    let hours = nanoseconds / 3_600_000_000_000_u128;
    nanoseconds %= 3_600_000_000_000_u128;
    let minutes = nanoseconds / 60_000_000_000_u128;
    nanoseconds %= 60_000_000_000_u128;
    let seconds = nanoseconds / 1_000_000_000_u128;
    nanoseconds %= 1_000_000_000_u128;
    let milliseconds = nanoseconds / 1_000_000_u128;
    format!(
        "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
        hours, minutes, seconds, milliseconds
    )
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
                        acc + &format!(
                            "{:0>2}/{:0>2}/{:0>4}: ",
                            date.day(),
                            date.month() as u8,
                            date.year()
                        ) + &ns_to_pretty_string(*duration)
                            + ", "
                    } else {
                        acc + &format!(
                            "{:0>2}/{:0>2}/{:0>4}: ",
                            date.day(),
                            date.month() as u8,
                            date.year()
                        ) + &ns_to_pretty_string(*duration)
                    }
                })
        )
    }
}

impl Blocs {
    pub(super) fn prune(&mut self, cutoff_date: &SyrDate) -> usize {
        let _tmp = self.len();
        self.retain(|key, _| key >= cutoff_date);
        _tmp - self.len()
    }
}
