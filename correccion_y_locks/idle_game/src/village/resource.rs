use strum_macros::{EnumIter, ToString};
use strum::IntoEnumIterator;

use crate::utils::random_number_in;

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

    pub fn random_variant() -> Self {
        let variant = match random_number_in(1, 3) {
            1 => Self::Wood,
            2 => Self::Stone,
            3 => Self::Silver,
            _ => Self::Wood,
        };

        variant
    }

    pub fn cost(&self) -> usize {
        let cost = match self {
            Self::Wood => 100,
            Self::Stone => 300,
            Self::Silver => 500,
        };

        cost
    }

    pub fn name_in_spa(&self) -> String {
        let name = match self {
            Self::Wood => "Madera".to_string(),
            Self::Stone => "Piedra".to_string(),
            Self::Silver => "Plata".to_string(),
        };

        name
    }

    pub fn name(&self) -> String {
        let name = match self {
            Self::Wood => "Wood".to_string(),
            Self::Stone => "Stone".to_string(),
            Self::Silver => "Silver".to_string(),
        };

        name
    }

    pub fn combine_resources(a_resource: Resource, another_resource: Resource) -> usize {
        a_resource.cost() + another_resource.cost() + random_number_in(-100 as isize, 300 as isize) as usize
    }
}

