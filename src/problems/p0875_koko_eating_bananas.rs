//! 875. Koko Eating Bananas
//! https://leetcode.com/problems/koko-eating-bananas/

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
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut lo, mut hi): (i32, i32) = piles
            .iter()
            .fold((piles[0], 0), |(minn, maxx), &p| (minn.min(p), maxx.max(p)));
        println!("{lo}, {hi}");
        lo = 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::can_eat(&piles, h, mid) {
                hi = mid;
            } else {
                lo = mid + 1
            }
        }
        lo
    }

    fn can_eat(piles: &[i32], h: i32, c: i32) -> bool {
        let mut used = 0;

        for &p in piles {
            used += ((p - 1) / c + 1) as i64;

            if used > h as i64 {
                return false;
            }
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
        Solution::min_eating_speed;
        // ex1: (vec![3, 6, 7, 11], 8) => 4,
        // ex2: (vec![30, 11, 23, 4, 20], 5) => 30,
        // ex3: (vec![30, 11, 23, 4, 20], 6) => 23,
        ex3: (vec![312884470], 312884469) => 2,
    }
}
