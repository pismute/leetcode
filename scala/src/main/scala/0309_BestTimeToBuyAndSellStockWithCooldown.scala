object BestTimeToBuyAndSellStockWithCooldown extends App {
  /*
    [1,2,3]

buy => sell, hold
sell => nopos
hold => sell, hold
nopos => buy, nopos

i v                          nopos:0:2
0 1.                 buy:-1:2              nopos
1 2          sell:1      hold:-1:3      ....
2 3.         nopos:1  sell:2:3  hold:-1
                             0
brute force

O(2^n), O(n)


dp: top down
O(n), O(n)

i v                      nopos:0 <-
0 1.         buy:-1. sell:?. hold:?, nopos:0  - base case
1 2          buy:-2  sell:1  hold:-1, nopos:0
2 3.         buy:-3  sell:2  hold:-1  nopos:1
                       nopos:2

buy[n]  = nopos[n-1] - p[n]
sell[n] = buy[n-1].max(hold[n-1]) + p[n]
hold[n] = buy[n-1].max(hold[n-1])
nopos[n] = nopos[n-1].max(sell[n-1])

O(n), O(1)

   */
  def maxProfit(prices: Array[Int]): Int = {

    def go(i: Int, buy: Int, sell: Int, hold: Int, nopos: Int): Int = {
      if (i >= prices.length) sell.max(nopos)
      else
        go(
          i + 1,
          nopos - prices(i),
          buy.max(hold) + prices(i),
          buy.max(hold),
          sell.max(nopos)
        )
    }

    val MinValue = -10000
    go(1, -prices(0), MinValue, MinValue, 0)
  }

  assert(maxProfit(Array(1, 2, 3, 0, 2)) == 3)
  assert(maxProfit(Array(1)) == 0)
}
