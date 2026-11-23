//! 104. Maximum Depth of Binary Tree
//! https://leetcode.com/problems/maximum-depth-of-binary-tree/

#![allow(unused_imports, unused_variables, unused_mut, dead_code, unreachable_code)]
use crate::util::*;
use crate::{cases, list, s, tree, vs, vvc};
use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                1 + Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::max_depth;
        // ex1: (root) => expected,
        ex1: (tree![3, 9, 20, null, null, 15, 7]) => 3,
        ex2: (tree![1, null, 2]) => 2,
        empty: (tree![]) => 0,
    }
}
