mod resource;
mod villager;

use std::{
    collections::HashMap, sync::{Arc, RwLock}, thread, time::Duration
};

use self::{
    resource::Resource,
    villager::Villager,
};

pub struct Village {
    gold: Arc<RwLock<usize>>,
    resources: Arc<RwLock<HashMap<String, usize>>>,
    villagers: usize,
}

impl Village {
    pub fn new(villagers: usize) -> Self {
        let starting_resources: HashMap<_, _> = Resource::variants()
            .into_iter()
            .map(|variant| (variant, 0))
            .collect();
        
        Village {
            gold: Arc::new(RwLock::new(0)),
            resources: Arc::new(RwLock::new(starting_resources)),
            villagers,
        }
    }

    pub fn start_activity(&mut self) { 
        let _: Vec<_> = (0..self.villagers)
        .map(|villager_number| {
            let total_gold_clone = Arc::clone(self.gold());
            thread::spawn(move || {
                let villager = Villager::new(villager_number);
                villager.work(total_gold_clone);
            })
        })
        .collect();

        loop {
            thread::sleep(Duration::from_secs(3));
            self.show_progress();
        }
    } 

    fn show_progress(&self) {
        println!("\n[VILLAGE PROGRESS]\n[GOLD]: {}\n[RESOURCES]: {:?}\n", self.gold().read().unwrap(), self.resources().read().unwrap());
    }

    pub fn gold(&self) -> &Arc<RwLock<usize>> {
        &self.gold
    }

    pub fn resources(&self) -> &RwLock<HashMap<String, usize>> {
        &self.resources
    }
}
