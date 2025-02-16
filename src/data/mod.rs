pub mod entries;
pub mod entry;
pub mod graphing;
pub mod syrtime;

pub use entries::Entries;
pub use entry::Entry;

pub enum IndexOptions {
    All,
    Indexed,
    Unindexed,
}
