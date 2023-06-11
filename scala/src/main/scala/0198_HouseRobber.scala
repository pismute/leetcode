object HouseRobber extends App {
  /*

[2,7,9,3,1]

                        2;[9,3,1];2                          1;[3,1]
               9;[1];11          3;                       3      4
               1;12

brute force
O(2^n), O(n)

caching
[2 [7 [9[3,1]]] ]
Array[house index] = amount of money.
0, 1,  2, 3, 4 (5)
12,10,10, 3, 1. 0

bottom up,

O(n), O(1)
0, 1, 2, 3, (4)
4, 3, 2, 2,   0
[2,1,1,2]

   */
  def rob(nums: Array[Int]): Int = {
    val s = Array(0, 0);

    for (i <- (nums.length - 1) to 0 by -1) {
      val max = s(0).max(s(1))
      s(i % 2) = nums(i) + s(i % 2)
      s((i + 1) % 2) = max
    }

    s(0).max(s(1))
  }

  assert(rob(Array(1, 2, 3, 1)) == 4)
  assert(rob(Array(2, 7, 9, 3, 1)) == 12)
}
