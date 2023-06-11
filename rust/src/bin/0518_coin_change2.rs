fn main() {
    assert_eq!(change(5, vec![1, 2, 5]), 4);
    assert_eq!(change(3, vec![2]), 0);
    assert_eq!(change(10, vec![10]), 1);
}
/*
 * 5, [1,2,5]
 *                                      0;5
 *                  1;4                      2;3            5;0
 *         1:3              2:2     ,   1:2      2:1
 *     1:2       2:1 ,  1:1    2:0  , 1:1 2;0 ,  1:0
 *   1:1  2:0  1:0   ,  1:0    1:0
 *   1:0
 *
 * brute force
 * O(c^n), O(n*n)
 * Set[subrray]
 *
 * brute force2 - once a coin is chosen, smaller coins cannot be chosen.
 *                                      0;5
 *                  1;4                      2;3            5;0
 *         1:3              2:2     ,        2:1
 *     1:2        ,         2:0     ,
 *   1:1  2:0     ,
 *   1:0
 *
 * O(c^n), O(n)
 *
 * Cache
 * array[coins][amount] =
 *   0 1 2 3 4 5 - index
 *   5 4 3 2 1 0
 * 1 4 3 2 2 1 1
 * 2 1 1 0 1 0 1
 * 5 1 0 0 0 0 1
 *   --
 * O(c*n), O(c*n)
 *
 * Cache2
 * O(c*n), O(n)
 *
 *   --
 */
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut a = vec![0; amount as usize + 1];

    a[amount as usize] = 1;

    for i in (0..coins.len()).rev() {
        for j in (0..amount as usize).rev() {
            let cur_amount = amount - j as i32;
            let c = coins[i];
            if cur_amount - c >= 0 {
                a[j] += a[j + c as usize];
            }
        }
    }

    a[0]
}
