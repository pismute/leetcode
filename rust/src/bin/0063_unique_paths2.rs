fn main() {
    assert_eq!(
        unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
    assert_eq!(unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
    assert_eq!(unique_paths_with_obstacles(vec![vec![1]]), 0);
}

/*
[[0,0,0],[0,1,0],[0,0,0]]

                                   (0,0)
                           (1,0)          (0,1)
                   (2,0)         (1, 1)X
                              ......

brute force
O(2^(n+m)), O(n+m)

dp: top down

| 0 1 2
0 1 1 1
1 1 X 1
2 1 1 2

O(n*m), O(n*m)

reuse array

O(n*m), O(n)
*/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; obstacle_grid[0].len() + 1];

    for i in 1..=obstacle_grid.len() {
        for j in 1..=obstacle_grid[0].len() {
            if i == 1 && j == 1 && obstacle_grid[0][0] == 0 {
                dp[1] = 1;
            } else if obstacle_grid[i - 1][j - 1] == 0 {
                dp[j] += dp[j - 1];
            } else {
                dp[j] = 0;
            }
        }
    }

    dp[obstacle_grid[0].len()]
}
