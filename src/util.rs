//! Shared LeetCode types and test helpers.
//!
//! Every problem file does `use crate::util::*;`, so everything here is in scope.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Linked list (identical to LeetCode's definition)
// ---------------------------------------------------------------------------

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Build a linked list from a slice: `[1, 2, 3]` -> 1 -> 2 -> 3.
pub fn to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

/// Convert a linked list back into a Vec (handy for asserts / printing).
pub fn list_to_vec(mut node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    while let Some(n) = node {
        out.push(n.val);
        node = &n.next;
    }
    out
}

// ---------------------------------------------------------------------------
// Binary tree (identical to LeetCode's definition)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

/// Build a tree from LeetCode's level-order format: `"[1,null,2,3]"`.
pub fn tree_from_str(s: &str) -> Tree {
    let vals: Vec<Option<i32>> = s
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|t| t.chars().filter(|c| !c.is_whitespace()).collect::<String>())
        .filter(|t| !t.is_empty())
        .map(|t| if t == "null" { None } else { Some(t.parse().expect("bad tree value")) })
        .collect();
    to_tree(&vals)
}

/// Build a tree from level-order values (`None` = null).
pub fn to_tree(vals: &[Option<i32>]) -> Tree {
    let root = Rc::new(RefCell::new(TreeNode::new((*vals.first()?)?)));
    let mut queue = VecDeque::from([root.clone()]);
    let mut it = vals[1..].iter();
    while let Some(node) = queue.pop_front() {
        for side in 0..2 {
            match it.next() {
                None => return Some(root),
                Some(None) => {}
                Some(Some(v)) => {
                    let child = Rc::new(RefCell::new(TreeNode::new(*v)));
                    queue.push_back(child.clone());
                    let mut n = node.borrow_mut();
                    if side == 0 { n.left = Some(child) } else { n.right = Some(child) }
                }
            }
        }
    }
    Some(root)
}

/// Convert a tree back to LeetCode's level-order format (trailing nulls trimmed).
pub fn tree_to_vec(root: &Tree) -> Vec<Option<i32>> {
    let mut out = vec![];
    let mut queue = VecDeque::from([root.clone()]);
    while let Some(node) = queue.pop_front() {
        match node {
            Some(n) => {
                let n = n.borrow();
                out.push(Some(n.val));
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            }
            None => out.push(None),
        }
    }
    while out.last() == Some(&None) {
        out.pop();
    }
    out
}

// ---------------------------------------------------------------------------
// Macros for writing test inputs quickly
// ---------------------------------------------------------------------------

/// `list![1, 2, 3]` -> `Option<Box<ListNode>>`
#[macro_export]
macro_rules! list {
    ($($x:expr),* $(,)?) => { $crate::util::to_list(&[$($x),*]) };
}

/// `tree![1, null, 2, 3]` -> `Option<Rc<RefCell<TreeNode>>>` (paste straight from LeetCode)
#[macro_export]
macro_rules! tree {
    ($($t:tt)*) => { $crate::util::tree_from_str(stringify!([$($t)*])) };
}

/// `s!("abc")` -> `String`
#[macro_export]
macro_rules! s {
    ($x:expr) => { String::from($x) };
}

/// `vs!["a", "b"]` -> `Vec<String>`
#[macro_export]
macro_rules! vs {
    ($($x:expr),* $(,)?) => { vec![$(String::from($x)),*] };
}

/// `vvc![["a","b"],["c","d"]]` -> `Vec<Vec<char>>` (grid problems)
#[macro_export]
macro_rules! vvc {
    ($([$($x:expr),* $(,)?]),* $(,)?) => {
        vec![$(vec![$($x.chars().next().unwrap()),*]),*]
    };
}

/// Table-driven tests. Each line becomes its own `#[test]`:
///
/// ```ignore
/// cases! {
///     Solution::two_sum;
///     ex1: (vec![2, 7, 11, 15], 9) => vec![0, 1],
///     ex2: (vec![3, 2, 4], 6)      => vec![1, 2],
/// }
/// ```
///
/// Prefix with `sorted` when answer order doesn't matter (expected and actual are both sorted).
#[macro_export]
macro_rules! cases {
    (sorted $f:path; $( $name:ident : ( $($arg:expr),* $(,)? ) => $expected:expr ),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let mut got = $f($($arg),*);
                let mut want = $expected;
                got.sort();
                want.sort();
                assert_eq!(got, want);
            }
        )*
    };
    ($f:path; $( $name:ident : ( $($arg:expr),* $(,)? ) => $expected:expr ),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($f($($arg),*), $expected);
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_roundtrip() {
        let t = tree![1, null, 2, -3];
        assert_eq!(tree_to_vec(&t), vec![Some(1), None, Some(2), Some(-3)]);
        assert_eq!(tree![], None);
    }

    #[test]
    fn list_roundtrip() {
        assert_eq!(list_to_vec(&list![1, 2, 3]), vec![1, 2, 3]);
    }
}
