# Restarting Leetcode once again with Two Sum

Because my last attempt at [Leetcode](https://leetcode.com) was, let's say, _polluted_ with others' solutions, not too many of the techniques I should have learned have stayed with me. With a view to correcting for that, I'm (re-)working through the [Grind 75](https://www.techinterviewhandbook.org/grind75?weeks=8&hours=8).

I'm going to offer a blow-by-blow as best I can to explain my reasoning as I work through this list. With that in mind, let's start at the start of the list with [Two Sum](https://leetcode.com/problems/two-sum).

For the uninitiated, Two Sum asks us:

<mark>Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.</mark>

```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

Fair enough.

Before looking at Leetcode's hints, you'd be forgiven for thinking it's easy (it can be! And Leetcode said it was!) with some kind of a brute force option:

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for x in (0..nums.len() - 1) {
            for y in (i + 1..nums.len()) {
                if nums[x] + nums[y] == target {
                    return vec![x as i32, y as i32];
                }
            }
        }

        vec![0, 0]
    }
}
```

This approach passes all the tests but suffers a time complexity of `O(n^2)` which is reflected in the stats on submitting the code:

```
Success
Runtime: 23 ms, faster than 29% of Rust online submissions for Two Sum.
Memory Usage: 2 MB, less than 99% of Rust Two Sum.
```

Not very impressive numbers on those boards. Let's see what the hints say.

## Hint one

This just told us about the brute force option for an initial solution, so let's cruise past it.

## Hint two

Leetcode tells us that:

> So, if we fix one of the numbers, say `x`, we have to scan the entire array to find the next number `y` which is `value - x` where value is the input parameter. Can we change our array somehow so that this search becomes faster?

Fix one number, and then we can scan the remainer for `target - x == y`? My intuition tells me that this is _barely_ different from our brute force implementation, especially since we account for skipping duplicates and "fixing" one number is another way of describing the outer loop. We _could_ sort the array and use two pointers in a left-right walk, but I'm not very interested in that approach.

## Hint three

> The second train of thought is, without changing the array, can we use additional space somehow? Like maybe a hash map to speed up the search?

This enables us to implement something like this:

```
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            match map.get(&(target - *num)){
                Some(&i) => return vec![idx as i32, i],
                None => map.insert(*num, idx as i32),
            };
        }

        vec![]
    }
}
```

This is much faster but uses more memory.

```
Success
Runtime: 3 ms, faster than 61% of Rust online submissions for Two Sum.
Memory Usage: 2.4 MB, less than 32% of Rust Two Sum.
```

## Further optimizations

Changing the `HashMap` to limit its capacity results in some decent gains on both measures.

```
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            match map.get(&(target - *num)){
                Some(&i) => return vec![idx as i32, i],
                None => map.insert(*num, idx as i32),
            };
        }

        vec![]
    }
}
```

To wit:

```
Success
Runtime: 0 ms, faster than 100% of Rust online submissions for Two Sum.
Memory Usage: 2.2 MB, less than 87% of Rust Two Sum.
```

At this point I'm not sure it's worth further optimizations, the `HashMap` probably uses some amount of memory which is unavoidable.
