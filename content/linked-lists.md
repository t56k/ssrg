# Linked lists

Linked lists, huh? _Groan_. Officially, the problem is called "Merge two sorted lists" but whatever. Let's get on with it.

> You are given the heads of two sorted linked lists `list1` and `list2`. Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists. Return the head of the merged linked list.

Example one:

```
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
```

## Sidebar

I've never understood (or can't remember) the point of this data structure, but since this Leetcode restart is about filling in gaps, let's find out. A quick search on Stack Overflow reveals [this](https://stackoverflow.com/questions/393556/when-to-use-a-linked-list-over-an-array-array-list) helpful thread (bookmarked!):

> Linked lists are preferable over arrays when:
>
>    1. you need constant-time insertions/deletions from the list (such as in real-time computing where time predictability is absolutely critical)
>
>    2. you don't know how many items will be in the list. With arrays, you may need to re-declare and copy memory if the array grows too big
>
>    3. you don't need random access to any elements
>
>    4. you want to be able to insert items in the middle of the list (such as a priority queue)
>
>    ...
>
>    Linked lists are really cheap to add or remove items anywhere and to iterate, but random access is O(n).

To wit:

```
Algorithm           ArrayList   LinkedList
seek front            O(1)         O(1)
seek back             O(1)         O(1)
seek to index         O(1)         O(N)
insert at front       O(N)         O(1)
insert at back        O(1)         O(1)
insert after an item  O(N)         O(1)
```

Okay, fine, maybe they have some utility. This knowledge might help us solve the problem anyway.

## Splicing 101

The problem gives us this definition for `ListNode`:

```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
```

Intuition tells us that to splice two lists together we need to iterate through the elements of the two lists and add them to a new resulting list, but not before comparing the elements to see which is greater.

```
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Allocate the result and make the two original lists mutable
        let mut head = ListNode::new(-1);
        let mut tail = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;

        // Nodes in both lists remaining (i.e., we have something to compare)
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        // If the lists do not _both_ have remaining nodes
        if list1.is_some() {
            tail.next = list1.take();
        }

        if list2.is_some() {
            tail.next = list2.take();
        }

        head.next
    }
}
```

This approach works, and is low on memory, but it ain't the fastest.

```
Success
Runtime: 2 ms, faster than 54% of Rust online submissions for Merge Two Sorted Lists.
Memory Usage: 2 MB, less than 95% of Rust Merge Two Sorted Lists.
```

I have a hunch that if we can avoid the allocation at the start by `match`ing the lists we might be able to speed up the solution. That's a fix for another day though.
