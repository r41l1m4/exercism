use crate::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

pub struct Allergies {
    allergens_list: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies: Vec<Allergen> = vec![];
        let mut score_tmp = score;

        for pow in (0..8u32).rev() {
            while score_tmp >= 256 {
                score_tmp -= 256
            };
            if score_tmp == 0 { break; }
            if score_tmp >= 2u32.pow(pow) {
                let (allergy, sc) = get_allergy_by_score(2u32.pow(pow));
                allergies.push(allergy);
                score_tmp -= sc;
            }
        }
        allergies.reverse();
        Self {allergens_list: allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens_list.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens_list.clone()
    }
}

fn get_allergy_by_score(score: u32) -> (Allergen, u32) {
    match score {
        128 => (Cats, 128),
        64 => (Pollen, 64),
        32 => (Chocolate, 32),
        16 => (Tomatoes, 16),
        8 => (Strawberries, 8),
        4 => (Shellfish, 4),
        2 => (Peanuts, 2),
        1 => (Eggs, 1),
        _ => (Eggs, 0),
    }
}

fn main() {
    let allergies = Allergies::new(248).allergies();
    let expected = &[
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    assert_eq!(&allergies, expected);
}