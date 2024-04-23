mod resource;
mod villager;

use std::{collections::HashMap, hash::Hash};
use resource::Resource;

use self::villager::Villager;
pub struct Village {
    gold: usize,
    resources: HashMap<String, usize>,
    villagers: usize,
}

impl Village {
    pub fn new(villagers: usize) -> Self {
        let starting_resources: HashMap<_, _> = Resource::variants()
            .into_iter()
            .map(|variant| (variant, 0))
            .collect();

        Village {
            gold: 0,
            resources: starting_resources,
            villagers,
        }
    }

    pub fn start_activity() {
        
    } 

    pub fn gold(&self) -> usize {
        self.gold
    }

    pub fn resources(&self) -> &HashMap<String, usize> {
        &self.resources
    }
}
