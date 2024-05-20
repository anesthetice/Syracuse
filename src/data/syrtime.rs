use std::collections::BTreeMap;
use anyhow::Context;
use serde::{Serialize, Deserialize, de::Visitor};
use itertools::Itertools;

// u128 representing nanoseconds
#[derive(Clone, Default, Serialize, Deserialize)]
pub(super) struct Blocs (BTreeMap<SyrDate, u128>);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyrDate(time::Date);

impl SyrDate {
    pub fn new(date: time::Date) -> Self {
        Self(date)
    }
    pub fn expand_from_bounds(start: Self, end: Self) -> Vec<Self> {
        let mut dates: Vec<SyrDate> = vec![start];
        let mut date = start;
        while date < end {
            date = date.next_day().unwrap_or(*end).into();
            dates.push(date)
        }
        dates
    }
}

impl std::fmt::Display for SyrDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{:0>2}/{:0>2}/{:0>4}",
            self.day(),
            self.month() as u8,
            self.year()
        )
    }
}

impl From<time::Date> for SyrDate {
    fn from(value: time::Date) -> Self {
        Self::new(value)
    }
}

impl TryFrom<&str> for SyrDate {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> 
    {
        let input: Vec<&str> = value.split('/').collect();
        if input.len() != 3 {
            Err(crate::error::Error{}).context("failed to parse date, invalid date format, expected dd/mm/yyyy")?;
        }
        Ok(Self::from(time::Date::from_calendar_date(
            input[2].parse::<i32>().context("failed to parse date, invalid year integer")?,
            time::Month::try_from(input[1].parse::<u8>().context("failed to parse date, invalid month integer, expected 01-12")?).context("failed to parse date, invalid month integer, expected 01-12")?,
            input[0].parse::<u8>().context("failed to parse date, invalid day integer, expected 01-31")?,
        ).context("failed to parse date, invalid date format, expected dd/mm/yyyy")?))
    }
}


impl Serialize for SyrDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for SyrDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_any(SyrDateVisitor)
    }
}

impl std::ops::Deref for SyrDate {
    type Target = time::Date;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SyrDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct SyrDateVisitor;

impl<'a> Visitor<'a> for SyrDateVisitor {
    type Value = SyrDate;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid string : dd/mm/yyyy")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        SyrDate::try_from(v).or(Err(E::custom("failed to parse date, invalid date format, expected dd/mm/yyyy")))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        SyrDate::try_from(v.as_str()).or(Err(E::custom("ffailed to parse date, invalid date format, expected dd/mm/yyyy")))
    }
}
