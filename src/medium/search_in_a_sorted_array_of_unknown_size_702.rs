// https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/

use crate::Solution;

impl Solution {
    /*
        There is no Rust version for this task. Here is Java impl:
        class Solution {
            public int search(ArrayReader reader, int target) {
                if (reader.get(0) == target) return 0;
                
                int left = 0;
                int right = 1;
                
                while (reader.get(right) < target) {
                    left = right;
                    right <<= 1;
                }
                
                while (left <= right) {
                    int pivot = left + ((right - left) >> 1);
                    int num = reader.get(pivot);

                    if (num == target) return pivot;
                    else if (num > target) right = pivot - 1;
                    else left = pivot + 1;
                }
                
                return -1;
            }
        }
    */
}
