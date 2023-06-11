object LongestPalindromicSubstring extends App {
  /*
    "babad"

    all substrings: n^2
    isPanlindrome: n

    O(n^3), O(1)

    isPanlidrome : n
    all of substring: n^2

     l
    "babad"
     |


    Determining panlindrome from middle.

    isPalinderome: N
    all possible substirng: N

    O(n^2) O(1)

   */
  def longestPalindrome(s: String): String = {

    def findPalindrome(l: Int, r: Int): (Int, Int) = {
      if (l < 0 || r >= s.length()) (l + 1, r - 1)
      else if (s.charAt(l) == s.charAt(r)) findPalindrome(l - 1, r + 1)
      else (l + 1, r - 1)
    }

    def go(i: Int, start: Int, to: Int): String = {
      if (i >= s.length()) s.substring(start, to + 1)
      else {
        val (x, y) = List(
          (start, to),
          findPalindrome(i, i + 1), // even
          findPalindrome(i, i) // odd
        ).maxBy { case (x1, y2) => y2 - x1 }

        go(i + 1, x, y)
      }
    }

    go(0, 0, 0)
  }

  assert(longestPalindrome("babad") == "bab")
  assert(longestPalindrome("cbbd") == "bb")
}
