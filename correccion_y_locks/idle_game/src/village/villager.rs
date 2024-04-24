type VillagerNumber = usize;

use std::{collections::HashMap, sync::{Arc, RwLock}, thread, time::Duration};

use crate::{
    utils::{
        coin_flip, 
        extract_gold
    }, 
    village::resource::Resource
};

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

    pub fn work (&self, village_gold: Arc<RwLock<usize>>, village_resources: Arc<RwLock<HashMap<String, usize>>>) {
        match self {
            Villager::Productor(number) => 
                Self::productor_work(*number, village_gold, village_resources),
            Villager::Consumer(number) => 
                Self::consumer_work(*number),
        }
    }

    fn productor_work(villager_number: usize, village_gold: Arc<RwLock<usize>>, village_resources: Arc<RwLock<HashMap<String, usize>>>) {
        loop {
            let extracted_gold = extract_gold();
            *village_gold.write().unwrap() += extracted_gold;
            println!("[ALDEANO {villager_number}] Soy productor y extraje {} oro.", extracted_gold);

            if coin_flip() == 0 {
                let resource = Resource::random_variant();

                if resource.cost() > *village_gold.read().unwrap() {
                    println!("[ALDEANO {villager_number}] Soy productor y decidi crear {}, pero no pude porque no hay suficiente oro.", resource.name_in_spa());
                } else {
                    *village_resources.write().unwrap().get_mut(&resource.name()).unwrap() += 1;
                    *village_gold.write().unwrap() -= resource.cost();
                    println!("[ALDEANO {villager_number}] Soy productor y decidi crear {}.", resource.name_in_spa());
                }
            } else {
                println!("[ALDEANO {villager_number}] Soy productor y decidi no crear recursos.");
            }
            thread::sleep(Duration::from_secs(1));
        }
        
    }

    fn consumer_work(villager_number: usize) {
        loop {
            if coin_flip() == 0 {
                println!("[ALDEANO {villager_number}] Soy consumidor y no hice nada");
                //Combinar recursos en oro
            } else {
                todo!()
                //Consumir recursos
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
