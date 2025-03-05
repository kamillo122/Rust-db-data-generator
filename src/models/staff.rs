use chrono::NaiveDate;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    pub name: String,
    pub department: String,
    pub salary: i32,
    pub phone: String,
    pub hire_date: NaiveDate,
}

impl Staff {
    pub fn load_names_from_file(filename: &str) -> Vec<String> {
        let file = File::open(filename).expect("Failed to open names file");
        let reader = BufReader::new(file);
        reader.lines().filter_map(Result::ok).collect()
    }

    pub fn generate_batch(count: usize, names: &[String]) -> Vec<Self> {
        let mut rng = thread_rng();
        let department_list = ["HR", "IT", "Finance", "Sales"];

        let mut used_phones = HashSet::new();
        let mut staff_list = Vec::with_capacity(count);

        for _ in 0..count {
            let name = names.choose(&mut rng).unwrap().clone();

            let phone = loop {
                let new_phone = format!(
                    "+48 {:08}",
                    rng.gen_range(6_000_000_000_i64..9_000_000_000_i64)
                );
                if used_phones.insert(new_phone.clone()) {
                    break new_phone;
                }
            };

            let salary = rng.gen_range(5_000..75_000);

            let hire_date = NaiveDate::from_ymd_opt(
                2017 + rng.gen_range(0..5),
                rng.gen_range(1..13),
                rng.gen_range(1..29),
            )
            .unwrap();

            staff_list.push(Self {
                name,
                department: department_list.choose(&mut rng).unwrap().to_string(),
                salary,
                phone,
                hire_date,
            });
        }

        staff_list
    }
}
