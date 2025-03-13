use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::utils;
use utils::utils::load_from_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

impl Client {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        // Wczytanie list imion i nazwisk
        let last_names: Vec<String> = load_from_file("last_names.txt");
        let first_names: Vec<String> = load_from_file("names.txt");

        let email_domains = vec!["gmail.com", "yahoo.com", "outlook.com", "example.com"];
        let mut used_phones = HashSet::new();
        let mut clients = Vec::new();

        for _ in 0..count {
            let first_name = first_names
                .choose(&mut rng)
                .cloned()
                .unwrap_or_else(|| "John".to_string());
            let last_name = last_names
                .choose(&mut rng)
                .cloned()
                .unwrap_or_else(|| "Doe".to_string());

            // Generowanie e-maila
            let email = format!(
                "{}.{}@{}",
                first_name.to_lowercase(),
                last_name.to_lowercase(),
                email_domains.choose(&mut rng).unwrap()
            );

            // Generowanie numeru telefonu w formacie +XX XXX XXX XXX
            let phone = loop {
                let new_phone = format!(
                    "+48 {:08}",
                    rng.gen_range(6_000_000_000_i64..9_000_000_000_i64)
                );
                if used_phones.insert(new_phone.clone()) {
                    break new_phone;
                }
            };

            clients.push(Client {
                first_name,
                last_name,
                email,
                phone,
            });
        }

        clients
    }
}
