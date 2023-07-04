fn main() {
    assert_eq!(distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
    assert_eq!(distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3), 7);
}

/*
 * [8, 15, 10, 20, 8]
 *                                                              (0,0)
 *                                 (8, 0)                                                       (0, 8)
 *                        (23, 0)                (8, 15)                             (15, 8)             (0, 23)
 *                  (33, 0)     (23, 10)  (18, 15)        (8, 25)           (25, 8)     (15, 18)   ....
 * brute force:
 * O(k^n), O(n)
 *
 */
pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    fn go(i: usize, cs: &[i32], cur: &mut [i32]) -> i32 {
        if i >= cs.len() {
            cur.iter().max().copied().unwrap()
        } else {
            let mut res = i32::MAX;
            for j in 0..cur.len() {
                cur[j] += cs[i];
                res = res.min(go(i + 1, cs, cur));
                cur[j] -= cs[i];
            }
            res
        }
    }

    go(0, &cookies, &mut vec![0; k as usize])
}
