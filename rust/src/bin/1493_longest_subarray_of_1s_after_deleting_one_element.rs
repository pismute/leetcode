fn main() {
    assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    assert_eq!(longest_subarray(vec![1, 0, 1, 1, 0, 1, 1, 1, 0]), 5);
    assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    assert_eq!(longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]), 4);
}

/*
 *
 * O(2n), O(1)
 */
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut flag = false;
    let mut max: i32 = 0;

    while i < nums.len() && j < nums.len() {
        if nums[j] == 0 {
            if flag {
                while i < j && nums[i] == 1 {
                    i += 1;
                }
                while i < j && nums[i] == 0 {
                    i += 1;
                }
            } else {
                flag = true;
            }
        }

        max = max.max(j as i32 - i as i32);
        j += 1;
    }

    max
}
