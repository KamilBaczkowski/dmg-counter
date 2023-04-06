use std::io;

mod parsers;

#[derive(Debug)]
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

    fn damage_probabilities(&self) -> Vec<(usize, f32)> {
        let possible_dmg = possible_roll_results(self.dice_rolls, self.dice_sides);

        let total_roll_count: usize = possible_dmg.iter().map(|(_, count)| count).sum();
        let probabilities = possible_dmg
            .iter()
            .map(|(dmg, count)| {
                (dmg + self.modifier, *count as f32 / total_roll_count as f32)
            })
            .collect();

        probabilities
    }
}

// Returns a set of (dmg, roll_count) given a number of rolls of a n sided die.
fn possible_roll_results(rolls: usize, sides: usize) -> Vec<(usize, usize)> {
    let mut possible_rolls = vec!();

    let minimum = rolls;
    let maximum = rolls * sides;
    let average = (minimum + maximum) / 2;

    let mut count = 1;
    for dmg in minimum..=maximum {
        possible_rolls.push((dmg, count));
        if dmg >= average {
            count -= 1;
        } else {
            count += 1;
        }
    }

    possible_rolls
}

fn main() {
    let mut weapon_1 = String::new();

    println!("Enter the number of dice and their sides for weapon 1 in the format 'xdy' or 'xdy+c': ");
    match io::stdin().read_line(&mut weapon_1) {
        Ok(_) => (),
        Err(e) => {
            println!("Weapon input malformed. Error: {:?}.", e);
            panic!();
        }
    }
    let weapon_1 = parsers::parse_weapon_input(weapon_1).unwrap();
    println!("{:?}", weapon_1.damage_probabilities());
}
