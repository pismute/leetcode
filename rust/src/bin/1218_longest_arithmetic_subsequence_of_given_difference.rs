fn main() {
    assert_eq!(longest_subsequence(vec![3, 4, -3, -2, -4], -5), 2);
    assert_eq!(longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    assert_eq!(longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
}

/*
 * [1,3,5,7]
 *
 * O(n), O(n)
 */
pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    type Map = std::collections::HashMap<i32, i32>;
    let mut dp = Map::new();

    let mut lst = 0;
    for i in 0..arr.len() {
        let nr = *dp.get(&(arr[i] - difference)).unwrap_or(&0) + 1;

        dp.insert(arr[i], nr);

        lst = lst.max(nr);
    }

    lst
}
