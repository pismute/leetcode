fn sort(mut xs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    xs.sort();
    xs
}

fn main() {
    assert_eq!(
        sort(subsets(vec![1, 2, 3])),
        sort(vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ])
    );
    assert_eq!(sort(subsets(vec![0])), sort(vec![vec![], vec![0]]));
}

/*
 * [1,2,3]
 *
 *                           []
 *               []                              3,[3]
 *         []              2           [];p[3]                2;[3,2]
 *     []     1;[1] [];[2]   1;[2,1]           [];[3,2]             ;[3,2,1]
 *
 * O(2^n), O(n)
 */
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len() * (nums.len() - 1) / 2;

    fn go(i: usize, ss: &mut Vec<i32>, ns: &[i32], mut acc: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if i >= ns.len() {
            acc.push(ss.clone());
            acc
        } else {
            // include
            ss.push(ns[i]);
            let acc2 = go(i + 1, ss, ns, acc);

            // not include
            ss.pop();
            go(i + 1, ss, ns, acc2)
        }
    }

    go(
        0,
        &mut Vec::with_capacity(nums.len()),
        &nums,
        Vec::with_capacity(len),
    )
}
