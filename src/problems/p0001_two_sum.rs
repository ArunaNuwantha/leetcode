//! 1. Two Sum
//! https://leetcode.com/problems/two-sum/

#![allow(unused_imports, unused_variables, unused_mut, dead_code, unreachable_code)]
use crate::util::*;
use crate::{cases, list, s, tree, vs, vvc};
use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&(target - n)) {
                return vec![j as i32, i as i32];
            }
            seen.insert(n, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::two_sum;
        // ex1: (nums, target) => expected,
        ex1: (vec![2, 7, 11, 15], 9) => vec![0, 1],
        ex2: (vec![3, 2, 4], 6) => vec![1, 2],
        ex3: (vec![3, 3], 6) => vec![0, 1],
    }
}
