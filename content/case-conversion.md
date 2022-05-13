# Case conversion

What with my being in clean-up mode, I realised the kebab-case blog post titles weren't all that they could be. I figured something like Ruby's `.to_sentence` would work better. In Rust, though, of course nothing like that exists. After all:

```
assert_eq!("Case conversion", "case-conversion".to_sentence_but_missing_fn());
```

A quick Google found [convert_case](https://docs.rs/convert_case/latest/convert_case/), but given that suitable conversions were only `Case::Title` or `Case::Lower` it wasn't quite what I wanted. Moreover, the fewer dependencies the better. Given that it's not exactly the most complex task, let's impl this thing ourselves.

It's pretty easy in Rust to replace hyphens with spaces:

```
let clean_file_name = "case-conversion".to_string();
let title = str::replace(clean_file_name, '-', " ");
```

Capitalising the first character, though, is another story. There's a decently-sized [thread](https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust) on this precise topic over at Stack Overflow, the depths of which I won't dive here. What I will do, though, is steal the solution:

```
fn titlize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
```

Accordingly, now we can simply call the fn on `title`:

```
titlize(&title);
```

And there we have it: prettier titles with no additional crates or CSS noise.
