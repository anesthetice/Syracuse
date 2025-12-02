use crate::data::syrtime::{SyrDate, TimeFormatting};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Blocks(pub BTreeMap<SyrDate, f64>);

impl std::ops::Deref for Blocks {
    type Target = BTreeMap<SyrDate, f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.iter()
                .sorted_by(|(a, _), (b, _)| { a.cmp(b) })
                .map(|(date, duration)| date.to_string() + "-" + &duration.s_str())
                .join(", ")
        )
    }
}
