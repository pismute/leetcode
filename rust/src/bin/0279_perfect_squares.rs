fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
/*
| root(12) = 3.xxxx
| [1, 4, 9]
|
|                    1;11           4;8           9
|                1;10, 4;7, 9   1   4;4    9
|            .....     ...        1 4 9
|       .....                      ...
|
|
| O(floor(root(n))^n), O(logn)
|
| memoization
| array[remaining number] = the minimal number of perfect squares.
|
| a[0] = 0
| a[1] = (a[1 - 1] + 1) = 1
| a[2] = (a[2 - 1] + 1) = 2
| a[3] = (a[3 - 1] + 1) = 3
| a[4] = (a[4 - 1] + 1).min(a[4 - 4] + 1) = 1
| ....
| a[n] = 3
|
| O(floor(root(n)) * n), O(n)
|
 */
pub fn num_squares(n: i32) -> i32 {
    let mut a = vec![10000; n as usize + 1];

    let squares: Vec<i32> = (1..=((n as f64).sqrt() as i32)).map(|x| x * x).collect();

    a[0] = 0;
    for i in 0..=n {
        for s in &squares {
            let ii = i - s;
            if ii >= 0 {
                a[i as usize] = a[i as usize].min(a[ii as usize] + 1);
            }
        }
    }

    a[n as usize]
}
