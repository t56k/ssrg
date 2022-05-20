# Oh, happy day: valid anagrams

I don't know what it is but the word-puzzle type problems really please me. I guess it's the Wordle addict in me. Who cares. Here we go:

> Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.
>
> An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

If that wasn't clear enough:

```
Input: s = "anagram", t = "nagaram"
Output: true
```

I like this very much.

## First pass

My initial idea is to convert each to `Vec`s of `chars()`, `sort()`, then check for `eq()`. Let's see how far that takes us.

```
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = Solution::sorted(s);
        let mut t = Solution::sorted(t);

        s.eq(&t)
    }

    fn sorted(s: String) -> Vec<char> {
        let mut res: Vec<char> = s.chars().collect();
        res.sort_by(|a, b| b.cmp(a));

        res
    }
}
```

Passes the test fine, let's see how she handles the corners.

```
Success
Runtime: 7 ms, faster than 35% of Rust online submissions for Valid Anagram.
Memory Usage: 2.7 MB, less than 6% of Rust Valid Anagram.
```

Ouch. It's be nice if my intuitions were better out of the gate, but then I guess I wouldn't be learning anything, and I wouldn't be able to explain my learning to y'all. Silver linings.

## Optimizations

I should try a `HashMap`. And maybe bytes.

```
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        for (a, b) in s.bytes().zip(t.bytes()) {
            *map.entry(a).or_insert(0) += 1;
            *map.entry(b).or_insert(0) -= 1;
        }

        map.values().all(|&c| c == 0)
    }
}
```

Gives us:

```
Success
Runtime: 3 ms, faster than 73% of Rust online submissions for Valid Anagram.
Memory Usage: 2.4 MB, less than 40% of Rust Valid Anagram.
```

Good enough!
