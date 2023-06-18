fn main() {
    assert_eq!(find_value_of_partition(vec![1, 3, 2, 4]), 1);
    assert_eq!(find_value_of_partition(vec![100, 1, 10]), 9);
}

/*
 * O(n), O(1)
 */
pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let mut min = i32::MAX;
    for w in (&nums).windows(2) {
        min = min.min(w[1] - w[0]);
    }

    min
}
