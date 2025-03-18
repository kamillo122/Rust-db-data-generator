use chrono::NaiveDate;
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String,
}

impl Project {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let names = vec!["Project A", "Project B", "Project C", "Project D"];
        let descriptions = vec![
            "A project focused on AI research.",
            "A new web development initiative.",
            "A marketing campaign for a new product.",
            "A system upgrade for internal software.",
        ];
        let statuses = vec!["Not Started", "In Progress", "Completed"];

        let mut projects = Vec::new();

        for _ in 0..count {
            let name = names
                .choose(&mut rng)
                .unwrap_or(&"Default Project")
                .to_string();
            let description = descriptions
                .choose(&mut rng)
                .unwrap_or(&"Default description")
                .to_string();

            let start_year = rng.gen_range(2022..2025);
            let end_year = rng.gen_range(start_year..2026);

            let start_date =
                NaiveDate::from_ymd_opt(start_year, rng.gen_range(1..=12), rng.gen_range(1..=28))
                    .expect("Invalid start date");
            let end_date =
                NaiveDate::from_ymd_opt(end_year, rng.gen_range(1..=12), rng.gen_range(1..=28))
                    .expect("Invalid end date");

            let status = statuses
                .choose(&mut rng)
                .unwrap_or(&"Not Started")
                .to_string();

            projects.push(Project {
                name,
                description,
                start_date,
                end_date,
                status,
            });
        }

        projects
    }
}
