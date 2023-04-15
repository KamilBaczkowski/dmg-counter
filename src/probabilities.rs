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
