use strum_macros::{EnumIter, ToString};
use strum::IntoEnumIterator;

#[derive(EnumIter, ToString)]
pub enum Resource {
    Wood,
    Stone,
    Silver,
}

impl Resource {
    pub fn variants() -> Vec<String> {
        Self::iter()
            .map(|v| v.to_string())
            .collect()
    }
}

