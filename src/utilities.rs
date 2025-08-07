use num::abs;
use rand_distr::{Distribution, Normal};

pub fn random_pile() -> u128 {
    let normal = Normal::new(10., 30.).unwrap();
    let v = abs(normal.sample(&mut rand::rng()));

    v as u128
}

pub fn get_column(x: f32) -> u128 {
    0
}
