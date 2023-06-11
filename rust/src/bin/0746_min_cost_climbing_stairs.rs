fn main() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}

/*
| [1,2,3,4,5]
| [1,[2,[3,[4,5]]]]
|
| brute force
| O(n * n), O(1)
|
| bottom up
| array[stair index] = min cost
| a[4] = 5
| a[3] = 4
| a[2] = 3 + min(a[3], a[4]) = 7
| a[1] = 2 + min(a[2], a[3]) = 6
| a[0] = 1 + min(a[1], a[2]) = 7
|
| O(n), O(1)
|
| top down
| a[0] = 1
| a[1] = 2
| a[2] = 3 + min(a[0], a[1]) = 4
| a[3] = 4 + min(a[1], a[2]) = 6
| a[4] = 5 + min(a[3], a[4]) = 9
|
| O(n), O(1)
 */

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut a = [0; 2];

    for i in 0..cost.len() {
        a[i % 2] = a[0].min(a[1]) + cost[i];
    }

    a[0].min(a[1])
}
