use dmg_counter::probabilities::probability;

#[test]
fn test_possible_roll_results_valid_with_one_die() {
    assert_eq!(probability(1, 1, 1), 1.0/1.0);

    assert_eq!(probability(1, 2, 1), 1.0/2.0);
    assert_eq!(probability(1, 2, 2), 1.0/2.0);

    assert_eq!(probability(1, 3, 1), 1.0/3.0);
    assert_eq!(probability(1, 3, 2), 1.0/3.0);
    assert_eq!(probability(1, 3, 3), 1.0/3.0);

    assert_eq!(probability(1, 4, 1), 1.0/4.0);
    assert_eq!(probability(1, 4, 2), 1.0/4.0);
    assert_eq!(probability(1, 4, 3), 1.0/4.0);
    assert_eq!(probability(1, 4, 4), 1.0/4.0);
}

#[test]
fn test_possible_roll_results_valid_with_two_dies() {
    assert_eq!(probability(2, 1, 2), 1.0/1.0);

    assert_eq!(probability(2, 2, 2), 1.0/4.0);
    assert_eq!(probability(2, 2, 3), 1.0/2.0);
    assert_eq!(probability(2, 2, 4), 1.0/4.0);

    assert_eq!(probability(2, 3, 2), 1.0/9.0);
    assert_eq!(probability(2, 3, 3), 2.0/9.0);
    assert_eq!(probability(2, 3, 4), 3.0/9.0);
    assert_eq!(probability(2, 3, 5), 2.0/9.0);
    assert_eq!(probability(2, 3, 6), 1.0/9.0);

    assert_eq!(probability(2, 4, 2), 1.0/16.0);
    assert_eq!(probability(2, 4, 3), 2.0/16.0);
    assert_eq!(probability(2, 4, 4), 3.0/16.0);
    assert_eq!(probability(2, 4, 5), 4.0/16.0);
    assert_eq!(probability(2, 4, 6), 3.0/16.0);
    assert_eq!(probability(2, 4, 7), 2.0/16.0);
    assert_eq!(probability(2, 4, 8), 1.0/16.0);
}

#[test]
fn test_possible_roll_results_valid_with_more_dies() {
    assert_eq!(probability(3, 3, 3), 1.0/27.0);
    assert_eq!(probability(3, 3, 4), 3.0/27.0);
    assert_eq!(probability(3, 3, 5), 6.0/27.0);
    assert_eq!(probability(3, 3, 6), 7.0/27.0);
    assert_eq!(probability(3, 3, 7), 6.0/27.0);
    assert_eq!(probability(3, 3, 8), 3.0/27.0);
    assert_eq!(probability(3, 3, 9), 1.0/27.0);

    // Now it becomes infeasible to test out everything.
    assert_eq!(probability(3, 4, 3), 1.0/64.0);
    assert_eq!(probability(3, 4, 7), 12.0/64.0);
    assert_eq!(probability(3, 4, 8), 12.0/64.0);
    assert_eq!(probability(3, 4, 12), 1.0/64.0);
    // 8^9/156 is around that, 156 is from
    // https://www.wolframalpha.com/input?i=9+dice+with+8+sides+that+result+in+55
    assert_eq!(probability(9, 8, 55), 863109.0/134217728.0);
}
