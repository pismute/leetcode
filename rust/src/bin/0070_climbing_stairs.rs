fn main() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut state = [1, 1];

    for i in (0..n as usize - 1).rev() {
        state[i % 2] += state[(i + 1) % 2]
    }

    state[0]
}
