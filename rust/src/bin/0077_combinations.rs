fn sort(mut xs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    xs.sort();
    xs
}

fn main() {
    assert_eq!(
        sort(combine(4, 2)),
        sort(vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4]
        ])
    );
    assert_eq!(combine(1, 1), vec![vec![1]]);
}

/*
 *
 * O(n*2^n), O(n*n)
 */
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn go(i: usize, cur: &mut Vec<i32>, nn: usize, kk: usize, acc: &mut Vec<Vec<i32>>) {
        if cur.len() == kk {
            acc.push(cur.clone());
        } else {
            for j in i..=nn {
                cur.push(j as i32);
                go(j + 1, cur, nn, kk, acc);
                cur.pop();
            }
        }
    }

    let mut res = vec![];
    go(1, &mut vec![], n as usize, k as usize, &mut res);
    res
}
