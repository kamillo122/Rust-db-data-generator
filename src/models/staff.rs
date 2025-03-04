use chrono::NaiveDate;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub department: String,
    pub salary: i32,
    pub phone: String,
    pub hire_date: NaiveDate,
}

impl Staff {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = thread_rng();
        let department_list = ["HR", "IT", "Finance", "Sales"];

        // Pre-generujemy dostępne wartości zamiast iterować w nieskończoność
        let mut available_ids: Vec<i32> = (1000..9999).collect();
        let mut available_names: Vec<String> =
            (1..=10_000).map(|i| format!("Staff_{}", i)).collect();
        let mut available_phones: Vec<String> = (1_000_000_000..1_000_000_000 + count as u64)
            .map(|num| format!("+48 {}", num))
            .collect();

        available_ids.shuffle(&mut rng);
        available_names.shuffle(&mut rng);
        available_phones.shuffle(&mut rng);

        let mut staff_list = Vec::with_capacity(count);

        for i in 0..count {
            let id = available_ids[i];
            let name = available_names[i].clone();
            let phone = available_phones[i].clone();

            let salary = rng.gen_range(30_000..100_000);

            let hire_date = NaiveDate::from_ymd_opt(
                2020 + rng.gen_range(0..5),
                rng.gen_range(1..13),
                rng.gen_range(1..29),
            )
            .unwrap();

            staff_list.push(Self {
                id,
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
