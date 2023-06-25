fn main() {
    assert_eq!(special_perm(vec![2, 3, 6]), 2);
    assert_eq!(special_perm(vec![1, 4, 3]), 2);
    assert_eq!(special_perm(vec![20, 100, 50, 5, 10, 70, 7]), 48);
    assert_eq!(
        special_perm(vec![
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192
        ]),
        178290591
    )
}

/*
 * [2, 3, 6]
 *                                    _;(0b0, 3)=2
 *               2;o(0b001:1, 0)=1               3;o(0b010:2, 1)=1                    6;o(0b100:4,2)=0
 *            3;x  6;o(0b101:5,2)=1        2;x          6;o(0b110:6,2)=1       2;o(0b101:5, 0)=0     3;o(0b110:6, 1)=0
 *                 3;o(0b111:7,1)=1                     2;o(0b111:7,0)=1
 *
 * O(n*2^n), O(n*2^n)
 */
pub fn special_perm(nums: Vec<i32>) -> i32 {
    fn go(mask: usize, cur: usize, ns: &[i32], dp: &mut Vec<Vec<i64>>) {
        if mask == ((1 << ns.len()) - 1) {
            dp[mask][cur] = 1;
        } else if dp[mask][cur] < 0 {
            let mut sum = 0;
            for i in 0..ns.len() {
                let pos = 1 << i;
                if mask & pos == 0
                    && (cur >= ns.len() || ns[cur] % ns[i] == 0 || ns[i] % ns[cur] == 0)
                {
                    go(mask | pos, i, ns, dp);
                    sum += dp[mask | pos][i];
                }
            }
            dp[mask][cur] = sum;
        }
    }

    let mut dp = vec![vec![-1; nums.len() + 1]; 1 << nums.len()];
    go(0, nums.len(), &nums, &mut dp);

    (dp[0][nums.len()] % (10_i64.pow(9) + 7)) as i32
}
