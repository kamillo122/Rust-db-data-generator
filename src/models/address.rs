use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::utils;
use utils::utils::load_from_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub postal_code: String,
}

impl Address {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let cities: Vec<String> = load_from_file("src/utils/cities.txt");
        let streets: Vec<String> = load_from_file("src/utils/streets.txt");

        let mut addresses = Vec::new();

        for _ in 0..count {
            if let (Some(city), Some(street)) = (cities.choose(&mut rng), streets.choose(&mut rng))
            {
                let street_number = rng.gen_range(1..=200).to_string();
                let postal_code = format!(
                    "{:02}-{:03}",
                    rng.gen_range(10..=99),
                    rng.gen_range(100..=999)
                );

                addresses.push(Address {
                    city: city.clone(),
                    street: street.clone(),
                    street_number,
                    postal_code,
                });
            }
        }

        addresses
    }
}
