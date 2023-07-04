import assertEqual from './assert';

function buddyStrings(s: string, goal: string): boolean {
  if (s.length != goal.length) {
    return false;
  } else {
    let j = 0;
    let dp = [-1, -1];
    for (let i = 0; i < s.length; i++) {
      if (j > 2) {
        break;
      }
      if (s[i] != goal[i]) {
        dp[j] = i;
        j += 1;
      }
    }

    return (j == 2 && s[dp[0]] == goal[dp[1]] && s[dp[1]] == goal[dp[0]]) ||
      (j == 0 && (new Set(s)).size != s.length);
  }
};

assertEqual(buddyStrings("abab", "abab"), true);
assertEqual(buddyStrings("a", "a"), false);
assertEqual(buddyStrings("aa", "bb"), false);
assertEqual(buddyStrings("ab", "ba"), true);
assertEqual(buddyStrings("ab", "ab"), false);
assertEqual(buddyStrings("aa", "aa"), true);




