use enum_iterator::{reverse_all, Sequence};

pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = Vec::new();

        // use greedy algorithm to find all the allergies, reduce to 0..255
        let mut remaining_score = score % 256;
        for allergen in reverse_all::<Allergen>() {
            if remaining_score >= allergen as u32 {
                remaining_score -= allergen as u32;
                allergies.push(allergen);
            }
        }

        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
