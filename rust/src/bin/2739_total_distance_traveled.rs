fn main() {
    assert_eq!(distance_traveled(5, 10), 60);
    assert_eq!(distance_traveled(1, 2), 10);
    assert_eq!(distance_traveled(9, 2), 110);
}

/*
 *
 * O(n), O(1)
 */
pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
    let mut cur = main_tank;
    let mut add = additional_tank;

    let mut spend = 0;
    while cur >= 5 && add > 0 {
        cur -= 4;
        spend += 5;
        add -= 1;
    }

    (spend + cur) * 10
}
