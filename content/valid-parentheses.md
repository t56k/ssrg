# Valid parentheses

Leetcode, question 20. Valid parentheses. Seems straightforward enough. Kinda.

> Given a string s containing just the characters `(`, `)`, `{`, `}`, `[` and `]`, determine if the input string is valid.
> An input string is valid if:
>   1. Open brackets must be closed by the same type of brackets.
>   2. Open brackets must be closed in the correct order.

Parsing unknown data is not for the faint of heart. So, scratch that, I'm going straight to the hints for this one.

## Hint one

> An interesting property about a valid parenthesis expression is that a sub-expression of a valid expression should also be a valid expression. (Not every sub-expression) e.g.
```
{ { } [ ] [ [ [ ] ] ] }  is VALID expression
          [ [ [ ] ] ]    is VALID sub-expression
  { } [ ]                is VALID sub-expression
```

> Can we exploit this recursive structure somehow?

I'm not convinced that the hint of a recursive strategy is quite enough here. Or, in other words, that's didn't really help me much. I'm not sure how to implement a recursive strategy at this point.

## Hint two

> What if whenever we encounter a matching pair of parenthesis in the expression, we simply remove it from the expression? This would keep on shortening the expression. e.g.
```
{ { ( { } ) } }
      |_|

{ { (      ) } }
    |______|

{ {          } }
  |__________|

{                }
|________________|

VALID EXPRESSION!
```

You can see the thinking between the two hints here. This makes sense. Yet, since we don't know a lot about the entire structure it is still a cumbersome and costly thing to parse. Onwards.

## Hint three

> The stack data structure can come in handy here in representing this recursive structure of the problem. We can't really process this from the inside out because we don't have an idea about the overall structure. But, the stack can help us process this recursively i.e. from outside to inwards.

[Bingo](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)). For those who've forgotten this data structure, it's essentially LIFO, and affords us `push()` and `pop()` operations--being that it mirrors the structure of the memory stack in your computer!

The logic of the solution runs roughly as follows.

If the current character (represented as bytes in the actual solution) is an opening parenthesis, we push that byte into the stack. If it's _anything else_ (i.e., a closing parenthesis), we subtract the previous byte (first out or popped [or if there's none then zero]) from the current byte. If the difference is greater than two, owing to their bytecodes (`() [] {} == 40 41 91 93 123 125`), we have an invalid `String`.

```
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut res = vec![];

        for b in s.into_bytes() {
            match b {
                b'{' | b'(' | b'[' => res.push(b),
                _ if (b - res.pop().unwrap_or(0)) > 2 => return false,
                _ => (),
            };
        }
        res.is_empty()
    }
}
```

Then, because we're popping the stack, we just need to determine if the stack is empty once the string is parsed.

```
Success
Runtime: 0 ms, faster than 100% of Rust online submissions for Valid Parentheses.
Memory Usage: 2.1 MB, less than 41% of Rust Valid Parentheses.
```
