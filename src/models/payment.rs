use chrono::{Datelike, NaiveDate, Utc};
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub amount: f32,
    pub payment_due_date: NaiveDate,
    pub method: String,
}

impl Payment {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let payment_methods = vec![
            "Credit Card",
            "Bank Transfer",
            "PayPal",
            "Cash",
            "Cryptocurrency",
        ];

        let mut payments = Vec::new();

        for _ in 0..count {
            let amount = rng.gen_range(10.0..=10_000.0);

            let start_year = Utc::now().year() - 3;
            let payment_due_date = NaiveDate::from_ymd_opt(
                rng.gen_range(start_year..=Utc::now().year()),
                rng.gen_range(1..=12),
                rng.gen_range(1..=28),
            )
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());

            let method = payment_methods.choose(&mut rng).unwrap().to_string();

            payments.push(Payment {
                amount,
                payment_due_date,
                method,
            });
        }

        payments
    }
}
