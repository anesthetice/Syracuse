use crate::data::{Blocs, Entry};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type UEntry = Entry<false>;

impl From<Entry<true>> for Entry<false> {
    fn from(value: Entry<true>) -> Self {
        Self::new(value.name, value.aliases, value.blocs)
    }
}

impl Serialize for Entry<false> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.name, &self.aliases, &self.blocs).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Entry<false> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (name, aliases, blocs) = <(String, Vec<String>, Blocs)>::deserialize(deserializer)?;
        Ok(Entry { name, aliases, blocs })
    }
}
