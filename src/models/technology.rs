use rand::rngs::SmallRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Technology {
    name: String,
    description: String,
}

impl Technology {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        let mut rng = SmallRng::from_entropy();

        let names = vec![
            "Rust Programming",
            "Machine Learning",
            "Blockchain",
            "Quantum Computing",
            "Artificial Intelligence",
            "Cloud Computing",
            "Internet of Things",
        ];

        let descriptions = vec![
            "A systems programming language focused on performance and safety.",
            "A subset of artificial intelligence that focuses on algorithms and models that allow machines to learn from data.",
            "A decentralized technology for secure and transparent transactions.",
            "A new field of computing that uses quantum mechanics to process information.",
            "A field of study in computer science that involves creating intelligent machines capable of performing tasks that usually require human intelligence.",
            "A model of computing where services and resources are provided over the internet.",
            "A network of physical devices, vehicles, buildings, and other objects embedded with sensors and software for the purpose of connecting and exchanging data.",
        ];

        let mut technologies = Vec::new();

        for _ in 0..count {
            let name = names
                .choose(&mut rng)
                .unwrap_or(&"Default Technology")
                .to_string();
            let description = descriptions
                .choose(&mut rng)
                .unwrap_or(&"Default description")
                .to_string();

            technologies.push(Technology { name, description });
        }

        technologies
    }
}
