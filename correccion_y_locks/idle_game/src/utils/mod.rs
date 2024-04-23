use super::constants::{MAX_GOLD, MIN_GOLD};
use rand::Rng;


pub fn coin_flip() -> usize {
    random_number_in(0, 1)
}

pub fn extract_gold() -> usize {
    random_number_in(MIN_GOLD, MAX_GOLD)
}

pub fn random_number_in(start: usize, finish: usize) -> usize {
    rand::thread_rng().gen_range(start..=finish)
}