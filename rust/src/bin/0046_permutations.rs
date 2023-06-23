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
    type Map = std::collections::HashMap<i32, i32>;
    fn insert(m: &mut Map, x: i32) {
        m.entry(x).and_modify(|v| *v += 1).or_insert(1);
    }

    fn remove(m: &mut Map, x: i32) {
        if let Some(v) = m.get_mut(&x) {
            if *v > 1 {
                *v -= 1;
            } else {
                m.remove(&x);
            }
        } else {
            m.remove(&x);
        }
    }

    fn go(cur: &mut Vec<i32>, m: &mut Map, acc: &mut Vec<Vec<i32>>) {
        if m.is_empty() {
            acc.push(cur.clone());
        } else {
            let keys = m.keys().copied().collect::<Vec<_>>();
            for k in keys {
                cur.push(k);
                remove(m, k);
                go(cur, m, acc);
                insert(m, k);
                cur.pop();
            }
        }
    }

    let mut map = nums
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            insert(&mut acc, x);
            acc
        });

    let mut res = vec![];
    go(&mut vec![], &mut map, &mut res);
    res
}
