use super::constants::{MAX_GOLD, MIN_GOLD};
use rand::Rng;


pub fn coin_flip() -> usize {
    rand::thread_rng().gen_range(0..=1)
}

pub fn extract_gold() -> usize {
    rand::thread_rng().gen_range(MIN_GOLD..=MAX_GOLD)
}