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
  function go(i: number, cur: number[], acc: number[][]) {
    res.push([...cur]);

    for (let j = i; j < nums.length; j++) {
      cur.push(nums[j]);
      go(j + 1, cur, res);
      cur.pop();
    }
  }

  const res: number[][] = [];
  go(0, [], res);
  return res;
};

function sort(arr: number[][]): number[][] {
  arr.sort();
  return arr;
}

assertEqual(subsets([1, 2, 3]), [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]);
assertEqual(subsets([0]), [[], [0]]);



