use crate::{probabilities::possible_roll_results};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Weapon {
    dice_rolls: usize,
    dice_sides: usize,
    modifier: usize,
}

impl Weapon {
    // fn min_damage(&self) -> usize {
    //     self.dice_rolls * 1 + self.modifier
    // }

    // fn avg_damage(&self) -> usize {
    //     (self.min_damage() + self.max_damage()) / 2
    // }

    // fn max_damage(&self) -> usize {
    //     self.dice_rolls * self.dice_sides + self.modifier
    // }

    pub fn damage_probabilities(&self) -> Vec<(usize, f32)> {
        let possible_dmg = possible_roll_results(self.dice_rolls, self.dice_sides).unwrap();

        let total_roll_count: usize = self.dice_sides.pow(self.dice_rolls as u32);
        let mut probabilities: Vec<(usize, f32)> = possible_dmg
            .iter()
            .map(|(dmg, count)| {
                (dmg + self.modifier, *count as f32 / total_roll_count as f32)
            })
            .collect();

        probabilities.sort_by(|(dmg_a, _), (dmg_b, _)| {
            dmg_a.cmp(dmg_b)
        });

        probabilities
    }
}

pub fn build_weapon(dice_rolls: usize, dice_sides: usize, modifier: usize) -> Weapon {
    Weapon { dice_rolls, dice_sides, modifier }
}
