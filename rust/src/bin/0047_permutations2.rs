fn sort(mut vv: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vv.sort();
    vv
}

fn main() {
    assert_eq!(
        sort(permute_unique(vec![1, 1, 2])),
        sort(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]])
    );
    assert_eq!(
        sort(permute_unique(vec![1, 2, 3])),
        sort(vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ])
    );
}

/*
 * [1, 1, 2]
 *
 * brute force:
 * O(n! or n*2^n), O(n)
 *
 * immutable approach with HashMap
 *
 * 0  : ([], {1,1,2})
 * 1st: ([1], {1, 2}),                ([2], {1, 1})
 * 2nd: ([1,1], {2}), ([1,2], {1}),   ([2, 1], {1})
 * 3nd: ([1,1,2], {}), ([1,2,1], {}), ([2,1,1], {})
 *
 * O(n*2^n), O(n*n)
 */
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    type Map = std::collections::HashMap<i32, i32>;

    fn remove(x: i32, mut m: Map) -> Map {
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

    let map = nums.iter().fold(Map::new(), |mut acc, x| {
        acc.entry(*x).and_modify(|c| (*c) += 1).or_insert(1);
        acc
    });

    fn go(cur: Vec<i32>, m: Map) -> Vec<Vec<i32>> {
        if m.is_empty() {
            vec![cur]
        } else {
            let mut res = vec![];
            for (k, _) in &m {
                let mut next = cur.clone();
                next.push(*k);
                res.extend(go(next, remove(*k, m.clone())));
            }
            res
        }
    }

    go(vec![], map)
}
