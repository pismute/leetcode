fn main() {
    assert_eq!(
        min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
}

/*
| [[1,3,1],[1,5,1],[4,2,1]]
|
|                               0,0
|                         0,1          (1,0)
|                     0,2    1,1
|                     1,2  1,2  2,1
|                     2,2  2,2  2,2
|
| brute force
| O(2^(n + m)), O(n+m)
|
| dp:2d: bottom up
| array[m][n] = min
|
| [[1,3,1],
|  [1,5,1],
|  [4,2,1]]
|
| array:
| [[7,6,3], M
|  [8,7,2], M
|  [7,3,1]] M
|  [M,M,M
| a[2][2] = 1
| a[2][1] = a[2][2].min(a[3][2]) + 2 = 3
| a[2][0] = a[2][1].min(a[3][1]) + 4 = 7
| a[1][2] = a[2][2].min(a[1][3]) + 2 = 3
| a[1][1] = a[1][2].min(a[2,1]) + 4 = 5
| ...
|
| O(n*m), O(n*m)
|
| dp:1d: top down
|
| [[1,3,1],
|  [1,5,1],
|  [4,2,1]]
|
| array:
| [1,4,5]
| [2,7,6]
| [6,8,7]
|
| a[j] = a[j].min(a[j-1]) + grid[i][j]
|
| O(n*m), O(n)
 */
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut a = vec![i32::MAX; grid[0].len() + 1];

    a[1] = 0;
    for i in 0..grid.len() {
        for j in 1..=grid[0].len() {
            a[j] = a[j].min(a[j - 1]) + grid[i][j - 1];
        }
    }

    a[grid[0].len()]
}
