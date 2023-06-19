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
 *  [1, 2, 3]
 *
 *                1              2               3
 *            2       3      1       3       1        2
 *            3       1      3       1       2        1
 *
 * O(n!), O(n)
 */
// pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut q = std::collections::VecDeque::from(nums);

//     fn go(ns: &mut std::collections::VecDeque<i32>) -> Vec<Vec<i32>> {
//         if ns.is_empty() {
//             vec![]
//         } else if ns.len() == 1 {
//             vec![ns.iter().copied().collect::<Vec<_>>()]
//         } else {
//             let mut res = vec![];
//             for _ in 0..ns.len() {
//                 let n = ns.pop_front().unwrap();
//                 let mut perms = go(ns);
//                 ns.push_back(n);

//                 for p in &mut perms {
//                     p.push(n);
//                 }

//                 res.extend(perms);
//             }
//             res
//         }
//     }

//     go(&mut q)
// }

/* immutable approach(just clone of mutable HashMap, in this case)
 *
 * 1st: ([1], {2, 3}),                ([2], {1, 3}),                ([3], {1, 2})
 * 2nd: ([1,2], {3}),  ([1,3], {2}),  ([2,1], {3}),  ([2,3], {1}),  ([3,1], {2}),  ([3,2], {1})
 * 3rd: ([1,2,3], {}), ([1,3,2], {}), ([2,1,3], {}), ([2,3,1], {}), ([3,1,2], {}), ([3,2,1], {})
 *
 * O(n*2^n or n!), O(n*(n-1))
 */
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    type Map = std::collections::HashMap<i32, i32>;
    fn insert(mut m: Map, x: i32) -> Map {
        m.entry(x).and_modify(|v| *v += 1).or_insert(1);
        m
    }

    fn remove(mut m: Map, x: i32) -> Map {
        if let Some(v) = m.get_mut(&x) {
            if *v > 1 {
                *v -= 1;
            } else {
                m.remove(&x);
            }
        } else {
            m.remove(&x);
        }

        m
    }

    let map = nums
        .into_iter()
        .fold(std::collections::HashMap::new(), insert);

    fn go(cur: Vec<i32>, m: Map) -> Vec<Vec<i32>> {
        if m.is_empty() {
            vec![cur]
        } else {
            let mut res = vec![];
            for (k, _) in &m {
                let mut next = cur.clone();
                next.push(*k);

                let perms = go(next, remove(m.clone(), *k));
                res.extend(perms);
            }
            res
        }
    }

    let mut res = vec![];
    for (k, v) in &map {
        for _ in 0..*v {
            let perms = go(vec![*k], remove(map.clone(), *k));
            res.extend(perms);
        }
    }
    res
}
