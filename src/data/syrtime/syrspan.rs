use jiff::{ToSpan, civil::Date};

use super::syrdate::SyrDate;

pub struct SyrSpan {
    /// The start date, inclusive
    pub start: Date,
    /// The end date, inclusive
    pub end: Date,
}

impl SyrSpan {
    pub fn from_start_and_end(start: Date, end: Date) -> Self {
        Self { start, end }
    }
    pub fn from_end_and_days_back(end: Date, days: i64) -> Self {
        Self {
            start: end.saturating_sub(days.days()),
            end,
        }
    }
}

impl std::fmt::Debug for SyrSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

pub struct SyrSpanIterator {
    pointer: Date,
    end: Date,
}

impl Iterator for SyrSpanIterator {
    type Item = SyrDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer <= self.end {
            let date = self.pointer.into();
            self.pointer = self.pointer.tomorrow().ok()?;
            Some(date)
        } else {
            None
        }
    }
}

impl IntoIterator for SyrSpan {
    type Item = SyrDate;
    type IntoIter = SyrSpanIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            pointer: self.start,
            end: self.end,
        }
    }
}
