pub struct Allergies(u32);

// Allergens defined as enum. Deriving essential Traits for the tests.
#[derive(Debug, PartialEq, Clone)]
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

// All possible allergens in a slice for easier iteration
static ALLERGENS: &'static [Allergen] = &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(n: u32) -> Self {
        Allergies(n)
    }

    pub fn is_allergic_to(&self, v: &Allergen) -> bool {
        self.0 & (v.clone() as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|v| self.is_allergic_to(v))
            .cloned()
            .collect()
    }
}
