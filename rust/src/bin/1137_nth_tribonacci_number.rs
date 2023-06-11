fn main() {
    assert_eq!(tribonacci(4), 4);
    assert_eq!(tribonacci(25), 1389537);
}

pub fn tribonacci(n: i32) -> i32 {
    let nn = n as usize;
    let mut s = [0, 1, 1];

    if nn < 3 {
        s[nn]
    } else {
        for i in 3..=nn {
            s[i % 3] = s.iter().sum()
        }

        s[nn % 3]
    }
}
