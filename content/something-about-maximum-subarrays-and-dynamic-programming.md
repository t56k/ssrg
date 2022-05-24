# Something about maximum subarrays and dynamic programming

Okay, back to basics.

> Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

Sure. Let's see what the first example says.

```
Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
```

Echoes of [Kadane](https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm).

# First pass

It's kinda straightforward enough. Loop through the `nums` `Vec` and check that each value is greater than zero. At that point we have two choices: either start again at the current index or add the current element to the previous sum.

```
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sub = vec![0; nums.len()]; // alloc subarray
        sub[0] = nums[0]; // assign init value for 1-len Vecs

        let mut res = sub[0]; // temp answer

        for i in 1..nums.len() {
            if sub[i - 1] > 0 {
                sub[i] = sub[i - 1] + nums[i];
            } else {
                sub[i] = nums[i];
            }

            res = res.max(sub[i]);
        }

        res
    }
}
```

```
Success
Runtime: 15 ms, faster than 61% of Rust online submissions for Maximum Subarray.
Memory Usage: 3.2 MB, less than 88% of Rust Maximum Subarray.
```

Pretty average as far as I'm concerned.

## Optimizations

```
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut tmp = 0i32;

        nums.iter().fold(nums[0], move |mut max, num| {
            tmp += num;
            max = max.max(tmp);
            tmp = tmp.max(0);

            max
        })
    }
}
```

```
Success
Runtime: 10 ms, faster than 91% of Rust online submissions for Maximum Subarray.
Memory Usage: 3.4 MB, less than 19% of Rust Maximum Subarray.
```

The window for memory differences ain't that big it seems.
