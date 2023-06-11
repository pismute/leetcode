fn main() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
    assert_eq!(coin_change(vec![1], 0), 0);
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let init = i32::MAX - 1;
    let mut state = vec![init; amount as usize + 1];
    state[0] = 0;
    for a in 1..=amount {
        for &c in &coins {
            let a2 = a - c;
            if a2 >= 0 {
                state[a as usize] = state[a as usize].min(state[a2 as usize] + 1)
            }
        }
    }

    let ret = state[amount as usize];
    if ret == init {
        -1
    } else {
        ret
    }
}
