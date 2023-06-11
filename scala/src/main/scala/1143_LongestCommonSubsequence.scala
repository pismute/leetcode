object LongestCommonSubsequence extends App {
  /*
    [a,b,c,d,e]
    [a,c,e]

                                (0,0)
                         (1,0)         (0,1)
                   (2,0)     (1,1)
                      ....

    val maxOfLCS

    O(2^(n + m)), O(logn)

    i  j a.  c.  e.
    a.   3.  2.  1
    b.   2.  2.  1
    c.   2.  2.  1
    d.   1.  1.  1
    e.   1.  1.  1.
             0
    O(n * m), O(n * m)

   */
  def longestCommonSubsequence(text1: String, text2: String): Int = {
    def get(i: Int, j: Int, dp: Array[Array[Int]]): Int = {
      if (i < 0 || j < 0 || i >= text1.length || j >= text2.length) 0
      else dp(i)(j)
    }

    def go(i: Int, j: Int, dp: Array[Array[Int]]): Int = {
      if (i < 0 || j < 0) dp(0)(0)
      else {
        if (text1.charAt(i) == text2.charAt(j)) {
          dp(i)(j) = get(i + 1, j + 1, dp) + 1
        } else {
          dp(i)(j) = get(i + 1, j, dp).max(get(i, j + 1, dp))
        }

        if (j == 0) go(i - 1, text2.length - 1, dp)
        else go(i, j - 1, dp)
      }
    }

    go(text1.length - 1, text2.length - 1, Array.ofDim[Int](text1.length, text2.length))
  }

  assert(longestCommonSubsequence("abcde", "ace") == 3)
  assert(longestCommonSubsequence("abc", "abc") == 3)
  assert(longestCommonSubsequence("abc", "def") == 0)
}
