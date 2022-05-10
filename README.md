# ssrg

## Rust static site generator

Stolen 100% from [Kerkour](https://kerkour.com/rust-static-site-generator) and changed to suit. I added Pico and used Warp instead of Tower and Axum. That's about all the differences.

```
mkdir public
cargo run
```

To publish you'd need to copy out the HTML files to wherever you wanna serve them from. I should remove Pico from CDN in due time too.
