use std::{collections::HashMap, sync::OnceLock};
use anyhow::Context;
use serde::{Serialize, Deserialize, de::Visitor};
use itertools::Itertools;

// u32 represents miliseconds, 8.64e7 miliseconds per day so u32 is perfectly fine
#[derive(Clone, Default, Serialize, Deserialize)]
pub(super) struct Blocs (HashMap<SyrDate, u32>);

impl std::ops::Deref for Blocs {
    type Target = HashMap<SyrDate, u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn ms_to_pretty_string(mut milliseconds: u32) -> String {
    let hours = milliseconds / 3600000;
    milliseconds %= 3600000;
    let minutes = milliseconds / 60000;
    milliseconds %= 60000;
    let seconds = milliseconds / 1000;
    milliseconds %= 1000;

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
                        ) + &ms_to_pretty_string(*duration)
                            + ", "
                    } else {
                        acc + &format!(
                            "{:0>2}/{:0>2}/{:0>4}: ",
                            date.day(),
                            date.month() as u8,
                            date.year()
                        ) + &ms_to_pretty_string(*duration)
                    }
                })
        )
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyrDate(time::Date);

impl SyrDate {
    pub fn new(date: time::Date) -> Self {
        Self(date)
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
