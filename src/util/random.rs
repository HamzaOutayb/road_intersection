use crate::models::prelude::*;

pub fn get_random_num(max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(0..=max)
}
