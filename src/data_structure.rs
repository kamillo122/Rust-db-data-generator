use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Staff {
    id: i32,
    name: String,
    department: String,
    salary: f64,
}

impl Staff {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen_range(1000..9999),
            name: format!("Staff_{}", rng.gen_range(1..100)),
            department: vec!["HR", "IT", "Finance", "Sales"][rng.gen_range(0..4)].to_string(),
            salary: rng.gen_range(30000.0..100000.0),
        }
    }
}
