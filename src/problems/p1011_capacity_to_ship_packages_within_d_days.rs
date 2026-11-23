//! 1011. Capacity To Ship Packages Within D Days
//! https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/

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
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut lo, mut hi) = weights.iter().fold((0, 0), |(m, s), &w| (m.max(w), s + w));
        println!("{lo}, {hi}");
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_ship(&weights, days, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    #[inline]
    fn can_ship(weights: &[i32], days: i32, margin: i32) -> bool {
        let mut current_days = 1;
        let mut summ = 0;
        println!("margin {margin}");
        for &w in weights {
            if summ + w > margin {
                current_days += 1;
                if current_days > days {
                    return false;
                }
                summ = 0;
            }
            summ += w;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::ship_within_days;
        ex1: (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5) => 15,
    }
}
