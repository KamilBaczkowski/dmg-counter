use crate::weapon::{Weapon, build_weapon};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum WeaponConstructError {
    MalformedInput,
    MalformedDiceCount,
    MissingDiceCount,
    MalformedDiceSidesCount,
    MissingDiceSidesCount,
    MalformedModifier
}

pub fn parse_weapon_input(input: String) -> Result<Weapon, WeaponConstructError> {
    let parts = input.trim();

    if parts.len() < 3 {
        // The shortest possible string is xdy, so if there are less chars than 3, then it is not a
        // valid input.
        return Err(WeaponConstructError::MalformedInput);
    }

    let mut parts = parts.split('d');

    // Get the number of dice thrown to determine the damage.
    let dice_rolls: usize = match parts.next() {
        Some(v) => {
            match v.parse() {
                Ok(v) => {
                    if v < 1 {
                        return Err(WeaponConstructError::MalformedDiceCount)
                    }
                    v
                },
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
                Ok(v) => {
                    if v < 1 {
                        return Err(WeaponConstructError::MalformedDiceSidesCount)
                    }
                    v
                },
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

    if parts.next().is_some() {
        // Two modifiers were added.
        return Err(WeaponConstructError::MalformedModifier);
    }

    Ok(build_weapon(dice_rolls, dice_sides, modifier))
}
