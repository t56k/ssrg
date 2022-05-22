# Flood fill, or bounds checking and grid work

Alright, this question's a little more complex than the string/array cakewalks we've been enjoying.

> An image is represented by an `m x n` integer grid image where `image[i][j]` represents the pixel value of the image.
>
> You are also given three integers `sr`, `sc`, and `newColor`. You should perform a flood fill on the image starting from the pixel `image[sr][sc]`.
>
> To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with `newColor`.
>
> Return the _modified image after performing the flood fill_.

Let's definitely look at the example _right now_:

<img src="img/flood1-grid.jpeg" alt="flood fill example 1" />

<br />
<br />

```
Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, newColor = 2
Output: [[2,2,2],[2,2,0],[2,0,1]]
Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.
```

Alright. I think I'm ready. Let's hop to it.

## First pass

Alright, bounds-checking is the first thing to solve for by grabbing the `len()` of the image and its first child element. Rust doesn't handle out of bounds issues gracefully. Not that it should.

```
// NOTE: Rows are `y`, cols are `x`
let y_limit = image.len();
let x_limit = image[0].len();
```

Now let's cast `x` and `y` to `u8` to ease our `Vec` access and add some safeguards to prevent needless flooding:

```
let y = sr as usize;
let x = sc as usize;

let old_color = image[y][x];
if old_color == new_color {
    return image;
}
```

This approach is going to build a `Vec` of tuples repesenting colors that will change on flooding starting with `sr`/`y` and `sc`/`x`. We also need a mutable reference to `image` so we can flood it without creating a new `image`.

```
let mut image = image;
let mut flood = vec![(y, x)];
```

And we can just `if-else` the rest [DFS-style](https://github.com/TheAlgorithms/Rust/blob/master/src/graph/depth_first_search.rs):

```
while !flood.is_empty() {
    if let Some(&(y, x)) = flood.last() {
        image[y][x] = new_color;

        // NOTE: up, down, left, right, out of bounds or other
        if y > 0 && image[y - 1][x] == old_color {
            flood.push((y - 1, x));
        } else if y + 1 < y_limit && image[y + 1][x] == old_color {
            flood.push((y + 1, x));
        } else if x > 0 && image[y][x - 1] == old_color {
            flood.push((y, x - 1));
        } else if x < x_limit - 1 && image[y][x + 1] == old_color {
            flood.push((y, x + 1));
        } else {
            flood.pop();
        }
    }
}
```

Works fine but it's not what we'd call performant.

```
Success
Runtime: 5 ms, faster than 30% of Rust online submissions for Flood Fill.
Memory Usage: 2.1 MB, less than 49% of Rust Flood Fill.
```

## Optimizations

Now this is *neater*, although I'm not really sure it's measurably better.

```
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let y_limit = image.len();
        let x_limit = image[0].len();

        let y = sr as usize;
        let x = sc as usize;

        let old_color = image[y][x];
        if old_color == new_color {
            return image;
        }

        let mut flood = vec![(y, x)];
        while let Some((y, x)) = flood.pop() {
            if image[y][x] != old_color {
                continue;
            }

            image[y][x] = new_color;

            if y > 0 {
                flood.push((y - 1, x));
            }

            if y < y_limit - 1 {
                flood.push((y + 1, x))
            }

            if x > 0 {
                flood.push((y, x - 1));
            }

            if x < x_limit - 1 {
                flood.push((y, x + 1));
            }
        }

        image
    }
}
```

Gives us:

```
Success
Runtime: 4 ms, faster than 57% of Rust online submissions for Flood Fill.
Memory Usage: 2.2 MB, less than 22% of Rust Flood Fill.
```

Not much better, although I'm starting to suspect Leetcode isn't the most consistent in its benchmarking given another run returns this:

```
Success
Runtime: 0 ms, faster than 100% of Rust online submissions for Flood Fill.
Memory Usage: 2.1 MB, less than 50% of Rust Flood Fill.
```

I should benchmark locally but I don't think the dataset is available without manually `println!()`ing it.
