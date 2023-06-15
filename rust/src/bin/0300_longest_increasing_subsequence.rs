fn main() {
    assert_eq!(length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}

/*
 * [0,1,0,3,2,3]
 *  2 2 2 2 2 2 = 2^n
 *
 *
 * brute force
 * O(2^n), O(n)
 *
 * dp: bottom up
 *
 * dp[i] = max(dp[i], if nums[i] > nums[j] { dp[j + 1] + 1} else { 1 }))
 *
 * [0,1,0,3,2,3]
 *  4 3 3 1 2 1
 *
 * O(n^2), O(n + 1)
 *
 * dp: top down
 *
 * [0,1,0,3,2,3]
 *  1 2 1 3 3 4
 *
 * dp[i] = max(dp[i], if num[i] > nums[j] { dp[j - 1] + 1 } else { 1 })
 *
 * O(n^2), O(n + 1)
 *
 * dp2: top down like LCS with sorted one
 *
 *   [0,0,1,2,3,3] - sorted
 * 0  1 1 1 1 1 1
 * 1  1 1 2 2 2 2
 * 0  1 2 2 2 2 2
 * 3  1 2 2 2 3 3
 * 2  1 2 2 3 3 3
 * 3  1 2 2 3 4 4
 *
 * O(n^2 + nlogn), O(2n)
 *
 */
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len() + 1];

    let mut max = 0;
    for i in 1..=nums.len() {
        for j in 1..=i {
            if nums[j - 1] < nums[i - 1] {
                dp[i] = dp[i].max(dp[j] + 1)
            } else {
                dp[i] = dp[i].max(1)
            }
        }
        max = max.max(dp[i]);
    }

    max
}
