use jiff::{civil::Date, Span};

use super::syrdate::SyrDate;

pub struct SyrSpan {
    pub start: Date,
    pub end: Date,
}

impl SyrSpan {
    pub fn from_start_and_end(start: Date, end: Date) -> Self {
        Self { start, end }
    }
    pub fn from_start_and_days_forward(start: Date, days: i64) -> Self {
        Self {
            start,
            end: start.saturating_add(Span::new().days(days)),
        }
    }
    pub fn from_end_and_days_back(end: Date, days: i64) -> Self {
        Self {
            start: end.saturating_sub(Span::new().days(days)),
            end,
        }
    }
    pub fn contains(&self, date: &Date) -> bool {
        (&self.start <= date) && (date <= &self.end)
    }
}

pub struct SyrSpanIterator {
    start: Date,
    end: Date,
    curr: Date,
}

impl Iterator for SyrSpanIterator {
    type Item = SyrDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.end {
            self.curr = self.curr.tomorrow().unwrap();
            Some(self.curr.into())
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
            start: self.start,
            end: self.end,
            curr: self.start,
        }
    }
}
