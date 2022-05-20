# Finally! Binary search

Okay, nuts and bolts time. This problem asks of us:

> Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, then return its index. Otherwise, return `-1`.
>
> You must write an algorithm with `O(log n)` runtime complexity.

The algoritm is a decent clue. Recall that an optimal (i.e., balanced) binary tree takes `O(log n)` time to resolve. The first example reads thus:

```
Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4
```

Easy if we're talking `O(n)`, but we're not. It's sorted already, so that's something.

## First pass

```
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut min, mut max) = (0, nums.len());

        while min < max {
            let i = (min + max) / 2;
            match nums[i].cmp(&target) {
                Ordering::Equal => return i as i32,
                Ordering::Less => min = i + 1,
                Ordering::Greater => max = i,
            }
        }

        -1
    }
}
```

Decent speed (comparatively), bad on memory.

```
Success
Runtime: 2 ms, faster than 93% of Rust online submissions for Binary Search.
Memory Usage: 2.4 MB, less than 15% of Rust Binary Search.
```

## Suboptimizations

A _plainer_ implementation doesn't save us much.

```
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut min, mut max) = (0, nums.len() + 1);

        while min < max {
            let i = (max + min) >> 1;

            if nums[i] == target { return i as i32; }

            if nums[i] < target {
                min = i + 1;
            } else {
                max = i;
            }
        }

        if nums[min] == target {
            min as i32
        }

        -1
    }
}
```

Results in:

```
Success
Runtime: 6 ms, faster than 47% of Rust online submissions for Binary Search.
Memory Usage: 2.5 MB, less than 15% of Rust Binary Search.
```

Not sure where else to take this one.
