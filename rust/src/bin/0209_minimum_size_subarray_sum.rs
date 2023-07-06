fn main() {
    assert_eq!(
        min_sub_array_len(213, vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12]),
        8
    );
    assert_eq!(min_sub_array_len(20, vec![2, 16, 14, 15]), 2);
    assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

/*
 *
 * O(n), O(1)
 */
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut len = usize::MAX;
    let mut sum = 0;
    let mut i = 0;

    for j in 0..nums.len() {
        sum += nums[j];
        while sum >= target {
            len = len.min(j - i + 1);
            sum -= nums[i];
            i += 1;
        }
    }

    if len == usize::MAX {
        0
    } else {
        len as i32
    }
}
