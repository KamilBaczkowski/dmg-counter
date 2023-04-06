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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_weapon_input_valid_inputs() {
        assert_eq!(parse_weapon_input(String::from("2d6+3")).unwrap(), build_weapon(2, 6, 3));
        assert_eq!(parse_weapon_input(String::from("1d20")).unwrap(), build_weapon(1, 20, 0));
        assert_eq!(parse_weapon_input(String::from("3d8+2")).unwrap(), build_weapon(3, 8, 2));
    }

    #[test]
    fn test_parse_weapon_input_invalid_inputs() {
        assert_eq!(parse_weapon_input(String::from("")), Err(WeaponConstructError::MalformedInput));
        assert_eq!(parse_weapon_input(String::from("d6")), Err(WeaponConstructError::MalformedInput));
        assert_eq!(parse_weapon_input(String::from("2")), Err(WeaponConstructError::MalformedInput));
        assert_eq!(parse_weapon_input(String::from("qwerqwerqwer")), Err(WeaponConstructError::MalformedDiceCount));
        assert_eq!(parse_weapon_input(String::from("2d6+")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("2d6+3+1")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("2d6-3")), Err(WeaponConstructError::MalformedDiceSidesCount));
    }

    #[test]
    fn test_parse_weapon_input_malformed_dice_counts() {
        assert_eq!(parse_weapon_input(String::from("ad6")), Err(WeaponConstructError::MalformedDiceCount));
        assert_eq!(parse_weapon_input(String::from("bd6")), Err(WeaponConstructError::MalformedDiceCount));
        assert_eq!(parse_weapon_input(String::from("0d6")), Err(WeaponConstructError::MalformedDiceCount));
        assert_eq!(parse_weapon_input(String::from("🫡d6")), Err(WeaponConstructError::MalformedDiceCount));
    }

    #[test]
    fn test_parse_weapon_input_missing_dice_sides() {
        assert_eq!(parse_weapon_input(String::from("15d")), Err(WeaponConstructError::MalformedDiceSidesCount));
        assert_eq!(parse_weapon_input(String::from("01d")), Err(WeaponConstructError::MalformedDiceSidesCount));
        assert_eq!(parse_weapon_input(String::from("131d")), Err(WeaponConstructError::MalformedDiceSidesCount));
    }

    #[test]
    fn test_parse_weapon_input_malformed_dice_sides() {
        assert_eq!(parse_weapon_input(String::from("15dqweqwe")), Err(WeaponConstructError::MalformedDiceSidesCount));
        assert_eq!(parse_weapon_input(String::from("15d0")), Err(WeaponConstructError::MalformedDiceSidesCount));
        assert_eq!(parse_weapon_input(String::from("01dbbbb")), Err(WeaponConstructError::MalformedDiceSidesCount));
        assert_eq!(parse_weapon_input(String::from("131d😇")), Err(WeaponConstructError::MalformedDiceSidesCount));
    }

    #[test]
    fn test_parse_weapon_input_malformed_modifier() {
        assert_eq!(parse_weapon_input(String::from("1d6+aaaaa")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("2d6+15a")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("3d1+🫠")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("1d6+aaaaa+123")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("2d6+15a+1")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("3d1+🫠+5")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("1d6+123+aaa")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("2d6+1+🫠")), Err(WeaponConstructError::MalformedModifier));
        assert_eq!(parse_weapon_input(String::from("3d1++5")), Err(WeaponConstructError::MalformedModifier));
    }
}
