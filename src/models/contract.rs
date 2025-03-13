use chrono::{Datelike, Duration, NaiveDate, Utc};
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    pub type_of_contract: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub salary: i32,
}

impl Contract {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();
        let type_of_contract_list = ["B2B", "UoP", "Mandate Contract", "Contract of Employment"];

        let mut contracts = Vec::new();

        for _ in 0..count {
            let type_of_contract = type_of_contract_list.choose(&mut rng).unwrap().to_string();

            let start_date = NaiveDate::from_ymd_opt(
                rng.gen_range(Utc::now().year() - 5..=Utc::now().year()),
                rng.gen_range(1..=12),
                rng.gen_range(1..=28),
            )
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());

            let contract_duration = Duration::days(rng.gen_range(180..=1825));
            let end_date = start_date + contract_duration;

            let salary = rng.gen_range(3000..=25000);

            contracts.push(Contract {
                type_of_contract,
                start_date,
                end_date,
                salary,
            });
        }

        contracts
    }
}
