fn sort(mut xs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    xs.sort();
    xs
}

fn main() {
    assert_eq!(
        sort(subsets_with_dup(vec![4, 4, 4, 1, 4])),
        sort(vec![
            vec![],
            vec![1],
            vec![1, 4],
            vec![1, 4, 4],
            vec![1, 4, 4, 4],
            vec![1, 4, 4, 4, 4],
            vec![4],
            vec![4, 4],
            vec![4, 4, 4],
            vec![4, 4, 4, 4]
        ])
    );
    assert_eq!(
        sort(subsets_with_dup(vec![1, 2, 2])),
        sort(vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ])
    );
    assert_eq!(sort(subsets_with_dup(vec![0])), sort(vec![vec![], vec![0]]));
}

/*
 * [1, 2, 2]
 *
 *               1;[1]                          2;[2]               2(X)
 *           2[1,2]          2(X)               2;[2,2]
 *           2[1,2,2]
 *
 *
 *
 * O(n!), O(n)
 */
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    fn go(i: usize, cur: &mut Vec<i32>, ns: &[i32], acc: &mut Vec<Vec<i32>>) {
        acc.push(cur.clone());

        for j in i..ns.len() {
            if j > i && ns[j - 1] == ns[j] {
                continue;
            }

            cur.push(ns[j]);
            go(j + 1, cur, ns, acc);
            cur.pop();
        }
    }

    let mut res = vec![];
    go(0, &mut vec![], &nums, &mut res);
    res
}
