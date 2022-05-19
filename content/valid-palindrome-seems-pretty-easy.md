# Valid palindrome seems pretty easy

Another day, another problem. Let's get palindromic!

> A phrase is a **palindrome** if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
>
> Given a string `s`, return `true` _if it is a palindrome_, _or_ `false` _otherwise_.

An example reads:

```
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
```

So, we can just compare a cleaned `Vec` of `char`s with its reverse. We don't have to worry about dividing by two or anything like that. Should save us some memory and time. Here we go.

```
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned_s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        cleaned_s.clone().eq(cleaned_s.rev())
    }
}
```

Well, that one felt good.

```
Success
Runtime: 0 ms, faster than 100% of Rust online submissions for Valid Palindrome.
Memory Usage: 2.3 MB, less than 75% of Rust Valid Palindrome.
```

Hard to be unhappy with this result. Not even going to try to optimize this one.
