use crate::{probabilities::{probability}};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Weapon {
    dice_rolls: usize,
    dice_sides: usize,
    modifier: usize,
}

impl Weapon {
    fn min_damage(&self) -> usize {
        self.dice_rolls + self.modifier
    }

    // fn avg_damage(&self) -> usize {
    //     (self.min_damage() + self.max_damage()) / 2
    // }

    fn max_damage(&self) -> usize {
        self.dice_rolls * self.dice_sides + self.modifier
    }

    pub fn damage_probabilities(&self) -> Vec<(usize, f32)> {
        (self.min_damage()..=self.max_damage())
            .map(|dmg| {
                (dmg, probability(self.dice_rolls, self.dice_sides, dmg))
            }).collect()
    }
}

pub fn build_weapon(dice_rolls: usize, dice_sides: usize, modifier: usize) -> Weapon {
    Weapon { dice_rolls, dice_sides, modifier }
}
