use std::{io};

mod parsers;
mod weapon;
mod probabilities;

fn main() {
    let mut weapon_1 = String::from("");

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
