fn sort(mut vv: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vv.sort();
    vv
}

fn main() {
    assert_eq!(
        sort(permute_unique(vec![2, 2, 1, 1])),
        sort(vec![
            vec![1, 1, 2, 2],
            vec![1, 2, 1, 2],
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
            vec![2, 1, 2, 1],
            vec![2, 2, 1, 1]
        ])
    );
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
 * 0  : ([], {1,1,2})
 * 1st: ([1], {1, 2}),                ([2], {1, 1})
 * 2nd: ([1,1], {2}), ([1,2], {1}),   ([2, 1], {1})
 * 3nd: ([1,1,2], {}), ([1,2,1], {}), ([2,1,1], {})
 *
 * O(n*2^n), O(n)
 */
pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    fn go(cur: &mut Vec<i32>, mask: usize, ns: &[i32], acc: &mut Vec<Vec<i32>>) {
        if cur.len() == ns.len() {
            acc.push(cur.clone());
        } else {
            for i in 0..ns.len() {
                let pos = 1 << i;
                let prev_pos = 1 << (i - 1);
                if mask & pos == 0 {
                    if i > 0 && ns[i] == ns[i - 1] && mask & prev_pos != 0 {
                        continue;
                    }
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
