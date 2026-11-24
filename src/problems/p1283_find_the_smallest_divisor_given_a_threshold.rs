//! 1283. Find the Smallest Divisor Given a Threshold
//! https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
//!
/*
Given an array of integers nums and an integer threshold, we will choose a positive integer divisor, divide all the array by it, and sum the division's result. Find the smallest divisor such that the result mentioned above is less than or equal to threshold.

Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: 7/3 = 3 and 10/2 = 5).

The test cases are generated so that there will be an answer.


Example 1:

Input:
let nums: Vec<i32> = vec![1, 2, 5, 9];
let threshold: i32 = 6;

Output: 5
Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).
Example 2:

Input:
let nums: Vec<i32> = vec![44, 22, 33, 11, 1];
let threshold: i32 = 5;

Output: 44


Constraints:

1 <= nums.length <= 5 * 104
1 <= nums[i] <= 106
nums.length <= threshold <= 106
*/

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
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut hi = nums.iter().fold(0, |m, &n| m.max(n));
        let mut lo = 1;

        println!("max {hi}");

        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::is_feasable(&nums, threshold, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn is_feasable(nums: &[i32], threshold: i32, c: i32) -> bool {
        let summ = nums.iter().fold(0, |s, &n| {
            let x: f64 = (n) as f64 / (c) as f64;
            s + x.ceil() as i32
        });
        println!("summ {c} {summ}");
        summ <= threshold
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::smallest_divisor;
        ex1: (vec![1, 2, 5, 9],6 ) => 5,
        ex2: (vec![44, 22, 33, 11, 1],5 ) => 44,
    }
}
