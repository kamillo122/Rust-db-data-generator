use chrono::NaiveDate;
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::utils::utils::load_from_file;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub position: String,
    pub contract_date: NaiveDate,
}

impl Employee {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let first_names: Vec<String> = load_from_file("src/utils/names.txt");
        let last_names: Vec<String> = load_from_file("src/utils/last_names.txt");
        let positions = [
            "HR",
            "IT",
            "Finance",
            "Sales",
            "Administration",
            "Public Relations",
        ];
        let email_domains = ["company.com", "corporate.com", "business.com"];

        let mut used_phones = HashSet::new();
        let mut employees = Vec::new();

        for _ in 0..count {
            let first_name = first_names
                .choose(&mut rng)
                .cloned()
                .unwrap_or_else(|| "John".to_string());
            let last_name = last_names
                .choose(&mut rng)
                .cloned()
                .unwrap_or_else(|| "Doe".to_string());

            let position = positions.choose(&mut rng).unwrap().to_string();
            let email = format!(
                "{}.{}@{}",
                first_name.to_lowercase(),
                last_name.to_lowercase(),
                email_domains.choose(&mut rng).unwrap()
            );

            let phone_number = loop {
                let new_phone = format!("+48 {}", rng.gen_range(600_000_000..999_999_999));
                if used_phones.insert(new_phone.clone()) {
                    break new_phone;
                }
            };

            let contract_date = NaiveDate::from_ymd_opt(
                rng.gen_range(2010..2025),
                rng.gen_range(1..=12),
                rng.gen_range(1..=28),
            )
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());

            employees.push(Employee {
                first_name,
                last_name,
                email,
                phone_number,
                position,
                contract_date,
            });
        }

        employees
    }
}
