fn main() {
    assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

/*
 * [0,3,1,4] with one transaction
 *
 * buy -> sell, hold
 * sell -> buy, nopos
 * hold -> sell, hold
 * nopos -> buy, nopos
 *
 *                                                                       nopos: 0
 * 0                                     buy:0                                                               nopos:0
 * 3                           sell:3                       hold:0                                        buy:-3                              nopos:0
 * 1                   buy:2        nopos:3           sell:1          hold:0                     sell:-2             hold:-3          buy:-1            nopos:0
 * 4            sell:6    hold:2  buy:-1 nopos:3   buy:-3 nopos:1  sell:4   hold:0          buy:-6   nopos:-2    sell:1   hold:-3  sell:3  hold:-1  buy:-4  nopos:0
 *
 * remove second transaction
 *                                                                       nopos: 0
 * 0                                     buy:0                                                               nopos:0
 * 3                           sell:3                       hold:0                                        buy:-3                              nopos:0
 * 1                                 nopos:3           sell:1          hold:0                     sell:-2             hold:-3          buy:-1            nopos:0
 * 4                              buy:-1 nopos:3          nopos:1  sell:4   hold:0                   nopos:-2    sell:1   hold:-3  sell:3  hold:-1  buy:-4  nopos:0
 *
 * brute force
 * O(2^n), O(n)
 *
 * bottom up
 *
 * buy1 -> sell1, hold1
 * sell1 -> buy2, nopos2
 * hold1 -> sell1, hold1
 * nopos1 -> buy1, nopos1
 * buy2 -> sell2, hold2
 * sell2 -> nopos3
 * hold2 -> sell2, hold2
 * nopos2 -> buy2, nopos2
 * nopos3 -> nopos3
 *                        6                      4
 *    buy1 sell1 hold1 nopos1 buy2 sell2 hold2 nopos2 nopos3
 * 0   6     3     6      3    4     0     4     3     0
 * 3   1     6     4      3    1     3     4     3     0
 * 1   3     1     4      0    3     1     4     0     0
 * 4   -4    4     0      0    -4    4     0     0     0
 *             0
 *
 * O(n*8), O(8)
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let last = prices[prices.len() - 1];
    let mut buy1 = -last;
    let mut sell1 = last;
    let mut hold1 = 0;
    let mut nopos1 = 0;
    let mut buy2 = -last;
    let mut sell2 = last;
    let mut hold2 = 0;
    let mut nopos2 = 0;
    let nopos3 = 0;
    /*
     * buy1 -> sell1, hold1
     * sell1 -> buy2, nopos2
     * hold1 -> sell1, hold1
     * nopos1 -> buy1, nopos1
     * buy2 -> sell2, hold2
     * sell2 -> nopos3
     * hold2 -> sell2, hold2
     * nopos2 -> buy2, nopos2
     * nopos3 -> nopos3
     */
    for i in (0..prices.len() - 1).rev() {
        let next_buy1 = sell1.max(hold1) - prices[i];
        let next_sell1 = buy2.max(nopos2) + prices[i];
        let next_hold1 = sell1.max(hold1);
        let next_nopos1 = buy1.max(nopos1);
        let next_buy2 = sell2.max(hold2) - prices[i];
        let next_sell2 = nopos3 + prices[i];
        let next_hold2 = sell2.max(hold2);
        let next_nopos2 = buy2.max(nopos2);

        buy1 = next_buy1;
        sell1 = next_sell1;
        hold1 = next_hold1;
        nopos1 = next_nopos1;
        buy2 = next_buy2;
        sell2 = next_sell2;
        hold2 = next_hold2;
        nopos2 = next_nopos2;
    }

    //nopos1
    buy1.max(nopos1)
}
