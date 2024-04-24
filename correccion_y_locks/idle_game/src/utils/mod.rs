use super::constants::{MAX_GOLD, MIN_GOLD};
use rand::Rng;


pub fn coin_flip() -> usize {
    random_number_in(0, 1) as usize
}

pub fn extract_gold() -> usize {
    random_number_in(MIN_GOLD as isize, MAX_GOLD as isize) as usize
}

pub fn random_number_in(start: isize, finish: isize) -> isize {
    rand::thread_rng().gen_range(start..=finish)
}