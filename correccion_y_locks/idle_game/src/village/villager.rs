type VillagerNumber = usize;


use std::{sync::{Arc, RwLock}, thread, time::Duration};

use crate::utils::{extract_gold, coin_flip};
pub enum Villager {
    Productor(VillagerNumber),
    Consumer(VillagerNumber),
}


impl Villager {
    pub fn new (villager_number: usize) -> Self {
        if coin_flip() == 0 {
            return Villager::Productor(villager_number);
        }

        Villager::Consumer(villager_number)
    }

    pub fn work (&self, village_gold: Arc<RwLock<usize>>) {
        match self {
            Villager::Productor(number) => 
                Self::productor_work(*number, village_gold),
            Villager::Consumer(number) => 
                Self::consumer_work(*number),
        }
    }

    fn productor_work(villager_number: usize, village_gold: Arc<RwLock<usize>>) {
        loop {
            println!("Soy el aldeano {villager_number}. Soy productor y estoy trabajando");
            let extracted_gold = extract_gold();

            *village_gold.write().unwrap() += extracted_gold;
            thread::sleep(Duration::from_secs(1));
        }
        
    }

    fn consumer_work(villager_number: usize) {
        loop {
            println!("Soy el aldeano {villager_number}. Soy consumidor y estoy trabajando");

            thread::sleep(Duration::from_secs(1));
        }
    }
}
