use itertools::Itertools;
use serde::{de::Visitor, Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    ops::{AddAssign, SubAssign},
    time::Duration,
};

use crate::utils::{duration_as_pretty_string, parse_date};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Entries(Vec<Entry>);

impl std::ops::Deref for Entries {
    type Target = Vec<Entry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Entries {
    const MAIN_FILEPATH: &'static str = "./syracuse.json";
    pub const BACKUPS_PATH: &'static str = "./backups/";

    pub fn load() -> anyhow::Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .read(true)
            .open(Self::MAIN_FILEPATH)?
            .read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(&buffer)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_vec_pretty(&self)?;
        Ok(std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(Self::MAIN_FILEPATH)?
            .write_all(&data)?)
    }

    pub fn backup(&self) -> anyhow::Result<()> {
        let data = serde_json::to_vec_pretty(&self)?;
        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp();
        let filepath: String = format!("{}syracuse-backup-{}.json", Self::BACKUPS_PATH, timestamp);
        Ok(std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filepath)?
            .write_all(&data)?)
    }

    pub fn search(&self, search_key: &str, threshold: f64) -> Vec<&Entry> {
        let search_key = search_key.to_uppercase();
        self.iter()
            .flat_map(|entry| {
                // returns the highest score found within the entry's names
                let max_score = entry
                    .names
                    .iter()
                    .map(|string| Entry::get_score(&search_key, string))
                    .fold(0.0, |max, x| if x > max { x } else { max });
                if max_score >= threshold {
                    Some((max_score, entry))
                } else {
                    None
                }
            })
            .sorted_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(_, entry)| entry)
            .collect()
    }

    pub fn clean(&mut self) {
        for entry in self.iter_mut() {
            entry
                .blocs
                .retain(|_, duration| *duration != Duration::ZERO)
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    /// an entry can have multiple names
    pub names: Vec<String>,
    /// keeps track of time spent when an entry is "active"
    pub blocs: Blocs,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.names
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (idx, slice)| {
                    if self.names.len() != idx + 1 {
                        acc + slice + ", "
                    } else {
                        acc + slice
                    }
                })
        )
    }
}

impl Entry {
    pub fn new(names: Vec<String>, blocs: Blocs) -> Self {
        Self { names, blocs }
    }

    pub fn is_name(&self, other_name: &String) -> bool {
        self.names.contains(other_name)
    }

    pub fn update_bloc_add(&mut self, date: &SyrDate, additional_duration: Duration) {
        if let Some(duration) = self.blocs.get_mut(date) {
            duration.add_assign(additional_duration);
        } else {
            self.blocs.insert(date.to_owned(), additional_duration);
        }
    }

    pub fn update_bloc_sub(&mut self, date: &SyrDate, reduced_duration: Duration) {
        if let Some(duration) = self.blocs.get_mut(date) {
            if *duration > reduced_duration {
                duration.sub_assign(reduced_duration);
            } else {
                duration.sub_assign(*duration)
            }
        } else {
            self.blocs.insert(date.to_owned(), Duration::ZERO);
        }
    }

    pub(self) fn get_score(search_key: &str, string: &str) -> f64 {
        search_key
            .chars()
            .zip(string.chars())
            .map(|(key, s)| if key == s { 1_u8 } else { 0_u8 })
            .sum::<u8>() as f64
            * (1.0 / search_key.len() as f64)
    }

    pub(super) fn get_points(&self, map: &HashMap<time::Date, usize>) -> Vec<(usize, f64)> {
        self.blocs
            .iter()
            .flat_map(|(date, duration)| {
                map.get(date)
                    .map(|x| (*x, duration.as_secs_f64() * (1.0 / 3600.0)))
            })
            .collect()
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Blocs(HashMap<SyrDate, Duration>);

impl std::ops::Deref for Blocs {
    type Target = HashMap<SyrDate, Duration>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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
                        ) + &duration_as_pretty_string(duration)
                            + ", "
                    } else {
                        acc + &format!(
                            "{:0>2}/{:0>2}/{:0>4}: ",
                            date.day(),
                            date.month() as u8,
                            date.year()
                        ) + &duration_as_pretty_string(duration)
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

impl From<time::Date> for SyrDate {
    fn from(value: time::Date) -> Self {
        Self::new(value)
    }
}

impl Serialize for SyrDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!(
            "{:0>2}/{:0>2}/{:0>4}",
            self.day(),
            self.month() as u8,
            self.year(),
        )
        .serialize(serializer)
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
        parse_date(v)
            .map(SyrDate::new)
            .ok_or(E::custom("invalid date, must be dd/mmy/yyyy"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        parse_date(&v)
            .map(SyrDate::new)
            .ok_or(E::custom("invalid date, must be dd/mmy/yyyy"))
    }
}

#[cfg(test)]
mod test {
    use super::{Blocs, Entries, Entry, SyrDate};
    use crate::utils::parse_date;

    #[test]
    fn get_score_1() {
        let search_key = "TEST";
        let string = "TESTING";

        assert_eq!(Entry::get_score(search_key, string), 1.0)
    }
    #[test]
    fn get_score_2() {
        let search_key = "TESTA";
        let string = "TESTING";

        assert_eq!(Entry::get_score(search_key, string), 0.8)
    }
    #[test]
    fn search_1() {
        let search_key = "TEST";
        let threshold: f64 = 0.55;
        let mut entries = Entries::default();
        entries.push(Entry::new(
            vec!["TESA".to_string(), "ABC".to_string()],
            Blocs::default(),
        ));
        entries.push(Entry::new(
            vec!["TEAB".to_string(), "ABST".to_string()],
            Blocs::default(),
        ));

        assert_eq!(
            entries.search(search_key, threshold)[0].names[0].as_str(),
            "TESA"
        )
    }
    #[test]
    fn search_2() {
        let search_key = "TEST";
        let threshold: f64 = 0.50;
        let mut entries = Entries::default();
        entries.push(Entry::new(
            vec!["TESA".to_string(), "ABC".to_string()],
            Blocs::default(),
        ));
        entries.push(Entry::new(
            vec!["TEAB".to_string(), "ABST".to_string()],
            Blocs::default(),
        ));

        assert_eq!(
            entries.search(search_key, threshold)[1].names[0].as_str(),
            "TEAB"
        )
    }
    #[test]
    fn search_3() {
        let search_key = "TEST";
        let threshold: f64 = 0.750001;
        let mut entries = Entries::default();
        entries.push(Entry::new(
            vec!["TESA".to_string(), "ABC".to_string()],
            Blocs::default(),
        ));
        entries.push(Entry::new(
            vec!["TEAB".to_string(), "ABST".to_string()],
            Blocs::default(),
        ));

        assert_eq!(entries.search(search_key, threshold).len(), 0)
    }
    #[test]
    fn syrdate_serialize() {
        let syrdate = SyrDate::new(parse_date("01/12/2023").unwrap());
        assert_eq!(serde_json::to_string(&syrdate).unwrap(), "\"01/12/2023\"")
    }
    #[test]
    fn syrdate_deserialize() {
        let syrdate = SyrDate::new(parse_date("01/12/2023").unwrap());
        let string_to_deserialize = "\"01/12/2023\"";

        assert_eq!(
            serde_json::from_str::<SyrDate>(string_to_deserialize).unwrap(),
            syrdate
        )
    }
}
