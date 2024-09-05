use super::number::Number;

use std::collections::HashMap;

#[derive(Clone, Debug, Default, Hash)]
pub struct Numbers([Number; 6]);

impl Numbers {
    const DUPLICATE_SMALLS_LIMIT: usize = 2;
    const DUPLICATE_LARGES_LIMIT: usize = 1;
    const REQUIRED_NUMBERS_COUNT: usize = 6;

    pub fn is_any_number_invalid(&self) -> bool {
        let valid_smalls = self
            .0
            .iter()
            .filter(|n| n.is_valid_small())
            .collect::<Vec<_>>();

        let valid_larges = self
            .0
            .iter()
            .filter(|n| n.is_valid_large())
            .collect::<Vec<_>>();

        valid_smalls.len() + valid_larges.len() != Self::REQUIRED_NUMBERS_COUNT
    }

    pub fn is_combination_invalid(&self) -> bool {
        let valid_counts = |numbers: &[&Number], limit: usize| {
            numbers
                .iter()
                .fold(HashMap::new(), |mut acc, n| {
                    *acc.entry((**n).clone()).or_insert(0) += 1;
                    acc
                })
                .iter()
                .filter(|(_, count)| **count > limit)
                .count()
                == 0
        };

        let valid_smalls = self
            .0
            .iter()
            .filter(|n| n.is_valid_small())
            .collect::<Vec<_>>();
        let all_smalls_valid = valid_counts(&valid_smalls, Self::DUPLICATE_SMALLS_LIMIT);

        let valid_larges = self
            .0
            .iter()
            .filter(|n| n.is_valid_large())
            .collect::<Vec<_>>();
        let all_larges_valid = valid_counts(&valid_larges, Self::DUPLICATE_LARGES_LIMIT);

        self.is_any_number_invalid() || !(all_smalls_valid && all_larges_valid)
    }

    pub fn is_valid(&self) -> bool {
        !(self.is_combination_invalid() || self.is_any_number_invalid())
    }

    pub fn update(&mut self, index: usize, number: &Number) {
        self.0[index] = number.clone()
    }
}

impl AsRef<[Number]> for Numbers {
    fn as_ref(&self) -> &[Number] {
        &self.0
    }
}
