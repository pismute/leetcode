fn main() {
    assert_eq!(longest_string(2, 5, 1), 12);
    assert_eq!(longest_string(3, 2, 2), 14);
    assert_eq!(longest_string(9, 9, 34), 104);
}

/*
 *
 *
 *
 *
 *
 *
 *
 */
pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
    let mi = x.min(y);
    let ma = x.max(y);
    let xy = ma - mi;
    if xy > 0 {
        (mi * 2 + z + 1) * 2
    } else {
        (mi * 2 + z) * 2
    }
}
