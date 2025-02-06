mod min_prefixes;
pub struct LocalPrefix {
    pub location: &'static str,
    pub content: &'static str,
    pub name: &'static str,
    pub title: &'static str,
}

pub const LOCAL_PREFIXES: &'static [LocalPrefix] = min_prefixes::LOCAL_PREFIXES;


