//! 1385. Find the Distance Value Between Two Arrays
//! https://leetcode.com/problems/find-the-distance-value-between-two-arrays/

#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code,
    unreachable_code
)]
use crate::util::*;
use crate::{cases, list, s, tree, vs, vvc};
use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut result = 0;
        let n = arr1.len();
        let m = arr2.len();

        for i in 0..n {
            let mut failed = false;
            for j in 0..m {
                if (arr1[i] - arr2[j]).abs() <= d {
                    failed = true;
                    break;
                }
            }

            if !failed {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::find_the_distance_value;
        // ex1: (nums) => expected,
        ex1: (vec![4,5,8], vec![10,9,1,8], 2) => 2
    }
}
