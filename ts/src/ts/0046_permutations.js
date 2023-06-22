"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var node_assert_1 = require("node:assert");
/*
 * [1,2,3]
 *
 *                 1                    2                           3
 *           2          3         1          3                 1         2
 *           3          2         3          1                 2         3
 *
 */
function permute(nums) {
    var map = new Map();
    for (var _i = 0, nums_1 = nums; _i < nums_1.length; _i++) {
        var n = nums_1[_i];
        map.
        ;
    }
    return [[1]];
}
;
(0, node_assert_1.default)(permute([1, 2, 3]) === [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);
(0, node_assert_1.default)(permute([0, 1]) === [[0, 1], [1, 0]]);
(0, node_assert_1.default)(permute([1]) === [[1]]);
