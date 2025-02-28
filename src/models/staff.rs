use chrono::NaiveDate;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub department: String,
    pub salary: f64,
    pub phone: String,
    pub hire_date: NaiveDate,
}

impl Staff {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = thread_rng();
        let department_list = ["HR", "IT", "Finance", "Sales"];

        let mut used_ids = HashSet::new();
        let mut used_names = HashSet::new();
        let mut used_phones = HashSet::new();

        let mut staff_list = Vec::with_capacity(count);

        for _ in 0..count {
            // Unikalne ID
            let id = loop {
                let new_id = rng.gen_range(1000..9999);
                if used_ids.insert(new_id) {
                    break new_id;
                }
            };

            // Unikalne imię
            let name = loop {
                let new_name = format!("Staff_{}", rng.gen_range(1..100));
                if used_names.insert(new_name.clone()) {
                    break new_name;
                }
            };

            // Unikalny telefon
            let phone = loop {
                let new_phone = format!(
                    "+48 {}{}{}-{}{}{}-{}{}{}",
                    rng.gen_range(1..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                    rng.gen_range(0..10),
                );
                if used_phones.insert(new_phone.clone()) {
                    break new_phone;
                }
            };

            // Pensja (zaokrąglenie do pełnych złotych)
            let salary = rng.gen_range(30000.0..100000.0_f64).round();

            // Data zatrudnienia
            let hire_date = NaiveDate::from_ymd_opt(
                2020 + rng.gen_range(0..5),
                rng.gen_range(1..13),
                rng.gen_range(1..29),
            )
            .unwrap();

            staff_list.push(Self {
                id,
                name,
                department: department_list[rng.gen_range(0..department_list.len())].to_string(),
                salary,
                phone,
                hire_date,
            });
        }

        staff_list
    }
}
