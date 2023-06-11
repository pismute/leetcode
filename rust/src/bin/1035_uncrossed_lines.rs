fn main() {
    assert_eq!(
        max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
    assert_eq!(
        max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}

/*  i
 * [2,5,1,2,5],    [2,1,5]
 * [10,5,2,1,5,2]  [2,1,5]
 *  j
 *                                                (0,0)
 *                                          (1,0)       (0,1
 *                                      (2,0)   (1,1)
 *
 *
 *
 *  brute force
 *  O(2^(n+m)), O(n+m)
 *
 *
 *  dp; bottom up
 *
 *     2 5 1 2 5
 * 10  3 3 2 2 1
 *  5  3 3 2 2 1
 *  2  3 2 2 2 1
 *  1  2 2 2 1 1
 *  5  2 2 1 1 1
 *  2  1 1 1 1 0
 *
 *  dp; top down
 *
 *     2 5 1 2 5
 * 10  0 0 0 0 0
 *  5  0 1 1 1 1
 *  2  1 1 1 2 2
 *  1  1 1 2 2 2
 *  5  1 2 2 2 3
 *  2  2 2 2 3 3
 *
 * O(n*m) O(n*m)
 *
 *
 * reuse array
 *
 * O(n*m) O(2n)
 */
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut a = vec![vec![0; nums1.len() + 1]; 2];

    for i in 1..=nums2.len() {
        for j in 1..=nums1.len() {
            if nums2[i - 1] == nums1[j - 1] {
                a[i % 2][j] = a[(i - 1) % 2][j - 1] + 1;
            } else {
                a[i % 2][j] = a[(i - 1) % 2][j].max(a[i % 2][j - 1]);
            }
        }
    }

    a[nums2.len() % 2][nums1.len()]
}
