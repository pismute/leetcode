fn main() {
    assert_eq!(count_beautiful_pairs(vec![31, 25, 72, 79, 74]), 7);
    assert_eq!(count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
    assert_eq!(count_beautiful_pairs(vec![11, 21, 12]), 2);
}

/*
 *
 */
pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn first(a: i32) -> i32 {
        let next = a / 10;
        if next == 0 {
            a
        } else {
            first(next)
        }
    }

    let mut nr = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let f = first(nums[i]);
            let l = nums[j] % 10;

            if gcd(f, l) == 1 {
                nr += 1
            }
        }
    }

    nr
}
