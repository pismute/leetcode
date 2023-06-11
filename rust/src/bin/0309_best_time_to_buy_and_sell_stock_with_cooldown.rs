fn main() {
    assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(max_profit(vec![1]), 0);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let last = prices.len() - 1;
    let mut buy = -prices[last];
    let mut sell = prices[last];
    let mut nopos = 0;
    let mut holding = 0;

    for i in (0..last).rev() {
        let p = prices[i];

        let next_buy = sell.max(holding) - p;
        let next_sell = nopos + p;
        let next_nopos = buy.max(nopos);
        let next_holding = sell.max(holding);

        buy = next_buy;
        sell = next_sell;
        nopos = next_nopos;
        holding = next_holding;
    }

    buy.max(nopos)
}
