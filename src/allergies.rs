pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => {
                return self.score % 2 == 1;
            },
            Allergen::Peanuts => true,
            Allergen::Shellfish => true,
            Allergen::Strawberries => true,
            Allergen::Tomatoes => true,
            Allergen::Chocolate => true,
            Allergen::Pollen => true,
            Allergen::Cats => true,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
