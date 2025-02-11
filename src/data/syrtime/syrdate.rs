use color_eyre::eyre::{bail, eyre, Context};
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyrDate(jiff::civil::Date);

impl SyrDate {
    pub fn new(date: jiff::civil::Date) -> Self {
        Self(date)
    }
    pub fn to_string_with_formatting(&self, sep_char: char) -> String {
        format!(
            "{:0>2}{sep_char}{:0>2}{sep_char}{:0>4}",
            self.day(),
            self.month(),
            self.year()
        )
    }
}

impl std::fmt::Display for SyrDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}/{:0>2}/{:0>4}",
            self.day(),
            self.month(),
            self.year()
        )
    }
}

impl From<jiff::civil::Date> for SyrDate {
    fn from(value: jiff::civil::Date) -> Self {
        Self::new(value)
    }
}

impl TryFrom<&str> for SyrDate {
    type Error = color_eyre::eyre::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_char = ['/', '.', '-', '_']
            .into_iter()
            .filter(|char| value.contains(*char))
            .nth(0)
            .ok_or_else(|| {
                eyre!("Failed to parse date, no separator character detected, ('/', '.', '-', '_')")
            })?;
        let input: Vec<&str> = value.split(split_char).collect();
        if input.len() != 3 {
            bail!("Failed to parse date, invalid date format, expected dd/mm/yyyy, or with '/' alternatives such as '.', '_', or '-'");
        }
        Ok(Self::from(jiff::civil::Date::new(
            input[2].parse::<i16>().context("Failed to parse date, invalid year")?,
            input[1].parse::<i8>().context("Failed to parse date, invalid month")?,
            input[0].parse::<i8>().context("Failed to parse date, invalid day")?,
        ).context("Failed to parse date, invalid date format, expected dd/mm/yyyy, or with '/' alternatives such as '.', '_', or '-'")?))
    }
}

impl TryFrom<&String> for SyrDate {
    type Error = color_eyre::eyre::Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
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
    type Target = jiff::civil::Date;
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
        SyrDate::try_from(v).or(Err(E::custom("Failed to parse date, invalid date format")))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        SyrDate::try_from(&v).or(Err(E::custom("Failed to parse date, invalid date format")))
    }
}
