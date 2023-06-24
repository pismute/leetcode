fn main() {
    let mut r1 = permute(vec![1, 2, 3]);
    r1.sort();
    let mut e1 = vec![
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ];
    e1.sort();
    assert_eq!(r1, e1);
    let mut r2 = permute(vec![0, 1]);
    r2.sort();
    let mut e2 = vec![[0, 1], [1, 0]];
    e2.sort();
    assert_eq!(r2, e2);
    let mut r3 = permute(vec![0, 1]);
    r3.sort();
    let mut e3 = vec![[0, 1], [1, 0]];
    e3.sort();
    assert_eq!(r3, e3);
}

/*
 * [1, 2, 3]
 *
 *               1               2              3
 *         2          3    1          3    1           2
 *         3          2    3          1    2           1
 *
 * O(n!), O(n)
 */
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn go(cur: &mut Vec<i32>, mask: usize, ns: &[i32], acc: &mut Vec<Vec<i32>>) {
        if cur.len() == ns.len() {
            acc.push(cur.clone());
        } else {
            for i in 0..ns.len() {
                let pos = 1 << i;
                if mask & pos == 0 {
                    cur.push(ns[i]);
                    go(cur, mask | pos, ns, acc);
                    cur.pop();
                }
            }
        }
    }

    let mut res = vec![];
    go(&mut vec![], 0, &nums, &mut res);
    res
}
