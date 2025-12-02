use crate::data::Blocs;

#[derive(Clone)]
pub struct DisplayEntry<'a> {
    pub name: &'a str,
    pub aliases: &'a [String],
    pub blocs: &'a Blocs,
    pub indexed: bool,
}
