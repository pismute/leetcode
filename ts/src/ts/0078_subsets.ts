import assertEqual from './assert';

/*
* [1,2,3]
*
*
*                      1                    2                    3
*                2           3                  3               
*                3
* O(n^2), O(n)
*/
function subsets(nums: number[]): number[][] {
  function go(i: number, cur: number[]): number[][] {
    let sum = [cur.slice()];

    for (let j = i; j < nums.length; j++) {
      cur.push(nums[j] as number);
      sum = sum.concat(go(j + 1, cur));
      cur.pop();
    }

    return sum;
  }

  return go(0, []);
};

assertEqual(subsets([1, 2, 3]), [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]);
assertEqual(subsets([0]), [[], [0]]);



