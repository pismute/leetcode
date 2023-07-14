fn main() {
    assert_eq!(
        Solution::can_finish(
            7,
            vec![
                vec![1, 0],
                vec![0, 3],
                vec![0, 2],
                vec![3, 2],
                vec![2, 5],
                vec![4, 5],
                vec![5, 6],
                vec![2, 4]
            ]
        ),
        true
    );
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![2, 0], vec![0, 2]]),
        false
    );
    assert_eq!(
        Solution::can_finish(
            13,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![2, 10],
                vec![3, 4],
                vec![4, 5],
                vec![4, 11],
                vec![5, 1]
            ]
        ),
        false
    );
}

struct Solution;

impl Solution {
    /*
     *
     * O(e*v), O(e)
     */
    pub fn can_finish(_num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        type Set = std::collections::HashSet<i32>;
        type Map = std::collections::HashMap<i32, Vec<i32>>;
        let mut edges = Map::new();
        let mut dp = Set::new();

        for e in &prerequisites {
            edges
                .entry(e[0])
                .and_modify(|xs| xs.push(e[1]))
                .or_insert(vec![e[1]]);
        }

        fn go(v: i32, visited: &mut Set, es: &Map, dp: &mut Set) -> bool {
            if dp.contains(&v) {
                true
            } else if visited.contains(&v) {
                false
            } else {
                visited.insert(v);
                if let Some(vs) = es.get(&v) {
                    for vv in vs {
                        if !go(*vv, visited, es, dp) {
                            return false;
                        }
                    }

                    dp.insert(v);
                    true
                } else {
                    dp.insert(v);
                    true
                }
            }
        }

        for &v in edges.keys() {
            if !go(v, &mut dp.clone(), &edges, &mut dp) {
                return false;
            }
        }

        true
    }
}
