fn main() {
    assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}

pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut perf = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());

    // O(n), O(n)
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for v in nums {
        perf.entry(v).and_modify(|x| *x += 1).or_insert(1);
        min = min.min(v);
        max = max.max(v);
    }

    let mut state = [0, 0];
    // O(n)
    for v in min..=max {
        state[(v as usize + 1) % 2] = state[0].max(state[1]);
        match perf.get(&v) {
            None => continue,
            Some(count) => {
                state[v as usize % 2] += v * count;
            }
        }
    }

    state[0].max(state[1])
}
