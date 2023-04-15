use dmg_counter::{parsers::{WeaponConstructError, parse_weapon_input}, weapon::build_weapon};

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
