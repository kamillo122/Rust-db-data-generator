use chrono::{Datelike, NaiveDate, Utc};
use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    amount: f32,
    payment_date: NaiveDate,
    method: String,
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
            // Losowa kwota płatności (od 10 do 10 000)
            let amount = rng.gen_range(10.0..=10_000.0);

            // Losowa data płatności w ciągu ostatnich 3 lat
            let start_year = Utc::now().year() - 3;
            let payment_date = NaiveDate::from_ymd_opt(
                rng.gen_range(start_year..=Utc::now().year()),
                rng.gen_range(1..=12),
                rng.gen_range(1..=28),
            )
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());

            // Losowa metoda płatności
            let method = payment_methods.choose(&mut rng).unwrap().to_string();

            payments.push(Payment {
                amount,
                payment_date,
                method,
            });
        }

        payments
    }
}
