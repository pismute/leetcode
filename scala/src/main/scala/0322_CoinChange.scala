object CoinChange extends App {
  type Amount = Int
  type NrOfCoins = Int
  def coinChange(coins: Array[Int], amount: Int): Int = {
    val results = scala.collection.mutable.Map.empty[Amount, NrOfCoins]

    def go(_amount: Int): Int =
      if (_amount < 0) -1
      else if (_amount == 0) 0
      else {
        results.getOrElse(
          _amount, {
            val min = coins.map(coin => go(_amount - coin) + 1).filter(_ > 0).minOption.getOrElse(-1)
            results(_amount) = min

            min
          }
        )
      }

    go(amount)
  }

  assert(coinChange(Array(1, 2, 5), 11) == 3)
  assert(coinChange(Array(2), 3) == -1)
  assert(coinChange(Array(1), 0) == 0)
}
