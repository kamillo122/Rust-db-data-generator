use chrono::NaiveDate;
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String,
}

impl Task {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let names = vec!["Task A", "Task B", "Task C", "Task D"];
        let descriptions = vec![
            "Task to research new technology.",
            "Task for setting up a new server.",
            "Task to write documentation.",
            "Task for a software code review.",
        ];
        let statuses = vec!["Not Started", "In Progress", "Completed"];

        let mut tasks = Vec::new();

        for _ in 0..count {
            let name = names
                .choose(&mut rng)
                .unwrap_or(&"Default Task")
                .to_string();

            let description = descriptions
                .choose(&mut rng)
                .unwrap_or(&"Default description")
                .to_string();

            let start_year = rng.gen_range(2022..2025);
            let end_year = rng.gen_range(start_year..2026);

            let start_date =
                NaiveDate::from_ymd_opt(start_year, rng.gen_range(1..=12), rng.gen_range(1..=28))
                    .unwrap();
            let end_date =
                NaiveDate::from_ymd_opt(end_year, rng.gen_range(1..=12), rng.gen_range(1..=28))
                    .unwrap();

            let status = statuses
                .choose(&mut rng)
                .unwrap_or(&"Not Started")
                .to_string();

            tasks.push(Task {
                name,
                description,
                start_date,
                end_date,
                status,
            });
        }

        tasks
    }
}
