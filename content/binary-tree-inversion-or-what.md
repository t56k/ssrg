# Binary tree inversion or what

Curveball:

> Given the `root` of a binary tree, invert the tree, and return its root.

I know we all remember the detail here, but as a refresher let's remind ourselves of the nuances of binary trees by seeing what Stack Overflow [has to say](https://stackoverflow.com/questions/2130416/what-are-the-applications-of-binary-trees). [This](https://stackoverflow.com/a/2159506/1153022) answer is pretty illuminating. Likewise that the [organization of Morse code is a binary tree](https://stackoverflow.com/a/31144242/1153022). Anyway:

<mark>**tl;dr**</mark>

> `0(log n)` searches

Got it.

Given that the implementation supplied with the question looks like this:

```
#[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
#[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
        val,
        left: None,
        right: None
        }
    }
}
```

It is more or less apparent that the question wants us to swap the `left` and `right` subnodes for each node. Who am I to deny them!

## First pass

```
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let (l, r) =
                    (Self::invert_tree(node.borrow().right.clone()),
                     Self::invert_tree(node.borrow().left.clone()));

                let node = node.clone();
                node.borrow_mut().left = l;
                node.borrow_mut().right = r;
                Some(node)
            },
            None => None,
        }
    }
}
```

Another bunt.

```
Success
Runtime: 2 ms, faster than 40% of Rust online submissions for Invert Binary Tree.
Memory Usage: 2 MB, less than 61% of Rust Invert Binary Tree.
```

## Optimizations

While it's not a great solution, I think most optimizations available will be language-specific and nothing algorithmic. Trading a `match` for `if let Some` might show some benefit.

```
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();

            let (l, r) = (
                Self::invert_tree(node.left.clone()),
                Self::invert_tree(node.right.clone())
            );

            node.left = r;
            node.right = l;
        }

        root
    }
}
```

```
Success
Runtime: 0 ms, faster than 100% of Rust online submissions for Invert Binary Tree.
Memory Usage: 2.1 MB, less than 63% of Rust Invert Binary Tree.
```

Hey ya.
