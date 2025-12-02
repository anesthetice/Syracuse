use crate::data::{Blocks, Entry};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type UEntry = Entry<false>;

impl From<Entry<true>> for UEntry {
    fn from(value: Entry<true>) -> Self {
        Self::new(value.name, value.aliases, value.blocks)
    }
}

impl Serialize for UEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.name, &self.aliases, &self.blocks).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (name, aliases, blocks) = <(String, Vec<String>, Blocks)>::deserialize(deserializer)?;
        Ok(Entry {
            name,
            aliases,
            blocks,
        })
    }
}
