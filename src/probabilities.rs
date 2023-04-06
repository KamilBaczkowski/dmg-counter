use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RollsError {
    InvalidInput
}

// Returns a set of (dmg, roll_count) given a number of rolls of a n sided die.
pub fn possible_roll_results(rolls: usize, sides: usize) -> Result<HashMap<usize, usize>, RollsError> {
    if rolls < 1 || sides < 1 {
        return Err(RollsError::InvalidInput);
    }

    let combinations: usize = sides.pow(rolls as u32);
    let mut possible_rolls = vec![vec![0; rolls]; combinations];

    let mut period = combinations;
    for i in 0..rolls {
        period /= sides;
        for (j, roll) in possible_rolls.iter_mut().enumerate() {
            roll[i] = (j/period) % sides + 1;
        }
    }

    let mut possible_sums = HashMap::new();
    for roll in possible_rolls {
        let iter = roll.into_iter();
        let sum = iter.sum::<usize>();
        let current: usize = *possible_sums.get(&sum).unwrap_or(&0);
        possible_sums.insert(sum, current + 1);
    }
    Ok(possible_sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_roll_results_valid_with_one_die() {
        let mut result = HashMap::new();
        result.insert(1, 1);
        assert_eq!(possible_roll_results(1, 1), Ok(result));
        let mut result = HashMap::new();
        result.insert(1, 1);
        result.insert(2, 1);
        assert_eq!(possible_roll_results(1, 2), Ok(result));
        let mut result = HashMap::new();
        result.insert(1, 1);
        result.insert(2, 1);
        result.insert(3, 1);
        assert_eq!(possible_roll_results(1, 3), Ok(result));
        let mut result = HashMap::new();
        result.insert(1, 1);
        result.insert(2, 1);
        result.insert(3, 1);
        result.insert(4, 1);
        assert_eq!(possible_roll_results(1, 4), Ok(result));
    }

    #[test]
    fn test_possible_roll_results_rejects_invalid_inputs() {

    }
}
