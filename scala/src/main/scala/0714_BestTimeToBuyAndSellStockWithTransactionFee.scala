object BestTimeToBuyAndSellStockWithTransactionFee extends App {
  /*
    [1, 2, 3] fee = 1

    i v                                        nopos:0
    0 1               buy:-2:0                                      nopos:0
    1 2.         sell:-1             hold:-2:2              buy:-3                nopos:0
    2 3      buy:-5   nopos:-1   sell:0:+2  hold:-2     sell:-1    hold:-3    buy:-4.  nopos:0
                                        0
brute force

O(2^n), O(n)

cache
HashMp[(Index, State), Profit]
O(n), O(n)


    i v                                        nopos:0
    0 1      buy:0   nopos:0.  sell:0. hold:2
    1 2.     buy:-1   nopos:0.  sell:1. hold:2
    2 3      buy:-4   nopos:0   sell:2  hold:0
                            0

bottom up.

O(n), O(1)

buy => sell, hold
sell => buy, nopos
hold => sell, hold
nopos => buy, nopos

   */
  def maxProfit(prices: Array[Int], fee: Int): Int = {
    def go(i: Int, buy: Int, nopos: Int, sell: Int, hold: Int): Int = {
      if (i < 0) nopos.max(buy)
      else {
        go(i - 1, sell.max(hold) - prices(i), buy.max(nopos), buy.max(nopos) + prices(i) - fee, sell.max(hold))
      }
    }

    val last = prices.length - 1
    go(last - 1, -prices(last), 0, prices(last) - fee, 0)
  }

  assert(maxProfit(Array(1, 3, 2, 8, 4, 9), 2) == 8)
  assert(maxProfit(Array(1, 3, 7, 5, 10, 3), 3) == 6)
}
