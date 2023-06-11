fn main() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut state = [0, 0];
    for i in 0..nums.len() {
        let cur: i32 = state[0].max(state[1]);
        state[i % 2] = cur.max(state[i % 2] + nums[i]);
        state[(i + 1) % 2] = cur.max(state[(i + 1) % 2]);
    }

    state[0].max(state[1])
}
