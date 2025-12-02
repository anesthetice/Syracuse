mod entries;
mod entry;
//mod graphing;
mod syrtime;

pub use entries::Entries;
pub use entries::IEntries;
pub use entries::UEntries;
pub use entry::AnyEntry;
pub use entry::Entry;
pub use entry::EntryCore;
pub use entry::IEntry;
pub use entry::UEntry;

pub use syrtime::Blocks;
pub use syrtime::SyrDate;
pub use syrtime::SyrSpan;
pub use syrtime::{TimeFormatting, WeekdayFormatting};

pub enum IndexOptions {
    All,
    Indexed,
    Unindexed,
}
