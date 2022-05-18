# Max profit, or: the contiguous subarray problem

[This](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) is a timely question, so it feels pertinent to answer it now.

> You are given an array `prices` where `prices[i]` is the price of a given stock on the `ith` day.
>
> You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
>
> Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return `0`.

Example one:

```
Input: prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
```

Sounds familiar. We want to return the greatest difference in the array going from lowest-left to highest-right. Sounds suspiciously like the [maximum contiguous subarray](https://en.wikipedia.org/wiki/Maximum_subarray_problem) problem. Let's try a first pass.

## First pass

```
use std::i32;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = i32::MAX;
        let mut res = 0i32;

        for price in prices {
            low = i32::min(low, price);
            res = i32::max(price - low, res);
        }

        res
    }
}
```

We set the `low` to the `MAX` value for the `i32` type, and the `res` at `0` (for the guard clause where no profit is available). Iterating through prices, we set `low` to the minimum of `low` and `price`, and each `res` to the maximum of the current price (less the cost) and the previous high. Because we only iterate once we necessarily move forward through time.

Unfortunately, we're not all that efficient here:

```
Success
Runtime: 26 ms, faster than 16% of Rust online submissions for Best Time to Buy and Sell Stock.
Memory Usage: 3 MB, less than 65% of Rust Best Time to Buy and Sell Stock.
```

Let's say we have a hunch that using `min()` and `max()` are causing the issues. Let's find out.

## Optimizations

```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = std::i32::MAX;
        let mut res = 0i32;

        for price in prices {
            if low > price {
                low = price;
            }

            let tmp = price - low;
            if tmp > res {
                res = tmp;
            }
        }

        res
    }
}
```

Waiting...

```
Success
Runtime: 33 ms, faster than 5% of Rust online submissions for Best Time to Buy and Sell Stock.
Memory Usage: 3 MB, less than 65% of Rust Best Time to Buy and Sell Stock.
```

Well, that's embarrassing.

We could `.fold()` the `Vec` instead of allocating `low` and `res` at the top of the fn.

```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, i32::MAX), |(mut res, mut low), price| {
                low = i32::min(*price, low);
                res = i32::max(res, price - low);
                (res, low)
            }).0
    }
}
```

Waiting...

```
Success
Runtime: 32 ms, faster than 5% of Rust online submissions for Best Time to Buy and Sell Stock.
Memory Usage: 2.9 MB, less than 65% of Rust Best Time to Buy and Sell Stock.
```

Less memory, which is to be expected.

What if we iterate over the length of `prices` instead of the elements?

```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = i32::MAX;
        let mut res = 0i32;

        for i in (0..prices.len()) {
            let curr = prices[i];

            low = i32::min(low, curr);
            res = i32::max(curr - low, res);
        }

        res
    }
}
```

Hmm.

```
Success
Runtime: 18 ms, faster than 50% of Rust online submissions for Best Time to Buy and Sell Stock.
Memory Usage: 3 MB, less than 65% of Rust Best Time to Buy and Sell Stock.
```

Still not _great_ but I'm not sure how to improve it at this point.
