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
 *                    1                   2                     3
 *               2           3                 3
 *               3
 *
 * O(2^n), O(n)
 */
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn go(i: usize, ss: &mut Vec<i32>, ns: &[i32]) -> Vec<Vec<i32>> {
        let mut acc = vec![ss.clone()];

        for j in i..ns.len() {
            ss.push(ns[j]);
            acc.extend(go(j + 1, ss, ns));
            ss.pop();
        }
        acc
    }

    go(0, &mut Vec::with_capacity(nums.len()), &nums)
}
