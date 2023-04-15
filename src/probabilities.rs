pub fn probability(dice_rolls: usize, dice_sides: usize, desired_outcome: usize) -> f32 {
    let total_outcomes = (dice_sides as f32).powi(dice_rolls as i32);
    let successful_outcomes = count_combinations(dice_rolls, dice_sides, desired_outcome) as f32;

    successful_outcomes / total_outcomes
}

fn count_combinations(dice_rolls: usize, dice_sides: usize, desired_outcome: usize) -> usize {
    // Track the highest possible number by rolling all highest.
    let max = dice_rolls * dice_sides;
    count_combinations_inner(dice_rolls, dice_sides, desired_outcome, max)
}

fn count_combinations_inner(
    dice_left: usize,
    dice_sides: usize,
    desired_outcome: usize,
    current_max: usize,
) -> usize {
    if dice_left == 1 {
        if desired_outcome >= 1 && desired_outcome <= dice_sides { return 1; } else { return 0; }
    }

    if desired_outcome > current_max {
        // If we roll max values, we won't match the target sum anyways, so lets bail.
        return 0;
    }

    if desired_outcome == current_max {
        // There is only one possible combination left (rolling only max values).
        return 1;
    }

    if desired_outcome < dice_left {
        // There is no way to roll the required amount, since even rolling only 1s will result in overflow.
        return 0;
    }

    let iter_max = if dice_sides > desired_outcome { desired_outcome - 1 } else { dice_sides };
    let mut count = 0;
    for current_value in 1..=iter_max {
        count += count_combinations_inner(
            dice_left - 1,
            dice_sides,
            desired_outcome - current_value,
            current_max - dice_sides,
        );
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_roll_results_valid_with_one_die() {
        assert_eq!(count_combinations(1, 1, 1), 1);

        assert_eq!(count_combinations(1, 2, 1), 1);
        assert_eq!(count_combinations(1, 2, 2), 1);

        assert_eq!(count_combinations(1, 3, 1), 1);
        assert_eq!(count_combinations(1, 3, 2), 1);
        assert_eq!(count_combinations(1, 3, 3), 1);

        assert_eq!(count_combinations(1, 4, 1), 1);
        assert_eq!(count_combinations(1, 4, 2), 1);
        assert_eq!(count_combinations(1, 4, 3), 1);
        assert_eq!(count_combinations(1, 4, 4), 1);
    }

    #[test]
    fn test_possible_roll_results_valid_with_two_dies() {
        assert_eq!(count_combinations(2, 1, 2), 1);

        assert_eq!(count_combinations(2, 2, 2), 1);
        assert_eq!(count_combinations(2, 2, 3), 2);
        assert_eq!(count_combinations(2, 2, 4), 1);

        assert_eq!(count_combinations(2, 3, 2), 1);
        assert_eq!(count_combinations(2, 3, 3), 2);
        assert_eq!(count_combinations(2, 3, 4), 3);
        assert_eq!(count_combinations(2, 3, 5), 2);
        assert_eq!(count_combinations(2, 3, 6), 1);

        assert_eq!(count_combinations(2, 4, 2), 1);
        assert_eq!(count_combinations(2, 4, 3), 2);
        assert_eq!(count_combinations(2, 4, 4), 3);
        assert_eq!(count_combinations(2, 4, 5), 4);
        assert_eq!(count_combinations(2, 4, 6), 3);
        assert_eq!(count_combinations(2, 4, 7), 2);
        assert_eq!(count_combinations(2, 4, 8), 1);
    }

    #[test]
    fn test_possible_roll_results_valid_with_more_dies() {
        assert_eq!(count_combinations(3, 3, 3), 1);
        assert_eq!(count_combinations(3, 3, 4), 3);
        assert_eq!(count_combinations(3, 3, 5), 6);
        assert_eq!(count_combinations(3, 3, 6), 7);
        assert_eq!(count_combinations(3, 3, 7), 6);
        assert_eq!(count_combinations(3, 3, 8), 3);
        assert_eq!(count_combinations(3, 3, 9), 1);

        // Now it becomes infeasible to test out everything.
        assert_eq!(count_combinations(3, 4, 3), 1);
        assert_eq!(count_combinations(3, 4, 7), 12);
        assert_eq!(count_combinations(3, 4, 8), 12);
        assert_eq!(count_combinations(3, 4, 12), 1);
        // 8^9/156 is around that, 156 is from
        // https://www.wolframalpha.com/input?i=9+dice+with+8+sides+that+result+in+55
        assert_eq!(863109, count_combinations(9, 8, 55));
    }
}
