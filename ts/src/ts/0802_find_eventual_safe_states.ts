import assertEqual from './assert';

/*
 * dfs
 * 
 * O(e+v), O(2v)
 */
function eventualSafeNodes(graph: number[][]): number[] {
  const safe = new Set<number>();
  const visited = new Set<number>();

  function go(i: number) {
    if (!safe.has(i) && !visited.has(i)) {
      visited.add(i);

      for (let v of graph[i]?.values()) {
        go(v);
        if (!safe.has(v)) return;
      }

      safe.add(i);
    }
  }

  for (let i = 0; i < graph.length; i++) {
    go(i)
  }

  const res: number[] = Array.from(safe);
  res.sort((a, b) => a - b)
  return res;
};

assertEqual(eventualSafeNodes([[3, 10], [5, 6, 14, 16], [], [6, 10, 14, 16, 19], [16], [9, 11, 16, 17], [8], [16, 19], [10, 13, 16, 17, 18, 19], [], [], [2, 12, 13, 16], [], [1, 14, 16, 17, 18], [0, 15, 16, 18, 19], [17, 19], [17, 18, 19], [18, 19], [7, 19], []]), [2, 9, 10, 12, 19])
assertEqual(eventualSafeNodes([[1, 2], [2, 3], [5], [0], [5], [], []]), [2, 4, 5, 6])
assertEqual(eventualSafeNodes([[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []]), [4])
