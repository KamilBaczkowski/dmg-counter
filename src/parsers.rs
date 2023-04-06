use crate::{Weapon};


#[derive(Debug)]
pub enum WeaponConstructError {
    MalformedInput,
    MalformedDiceCount,
    MissingDiceCount,
    MalformedDiceSidesCount,
    MissingDiceSidesCount,
    MalformedModifier
}

pub fn parse_weapon_input(input: String) -> Result<Weapon, WeaponConstructError> {
    let mut parts = input.trim().split('d');

    // Get the number of dice thrown to determine the damage.
    let dice_rolls: usize = match parts.next() {
        Some(v) => {
            match v.parse() {
                Ok(v) => v,
                Err(_) => return Err(WeaponConstructError::MalformedDiceCount),
            }
        },
        None => return Err(WeaponConstructError::MissingDiceCount),
    };

    // This should get the rest of the string.
    let rest = match parts.next() {
        Some(v) => v,
        None => return Err(WeaponConstructError::MissingDiceSidesCount),
    };

    // If something is left, then the input was malformed for sure.
    if parts.next().is_some() {
        return Err(WeaponConstructError::MalformedInput);
    }
    let mut parts = rest.split('+');

    let dice_sides: usize = match parts.next() {
        Some(v) => {
            match v.parse() {
                Ok(v) => v,
                Err(_) => return Err(WeaponConstructError::MalformedDiceSidesCount),
            }
        },
        None => return Err(WeaponConstructError::MissingDiceSidesCount),
    };

    let modifier: usize = match parts.next() {
        Some(v) => {
            match v.parse() {
                Ok(v) => v,
                Err(_) => return Err(WeaponConstructError::MalformedModifier),
            }
        },
        None => 0,
    };

    Ok(Weapon { dice_rolls, dice_sides, modifier })
}
