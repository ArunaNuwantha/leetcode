//! 2187. Minimum Time to Complete Trips
//! https://leetcode.com/problems/minimum-time-to-complete-trips/
//!
/*
You are given an array time where time[i] denotes the time taken by the ith bus to complete one trip.

Each bus can make multiple trips successively; that is, the next trip can start immediately after completing the current trip. Also, each bus operates independently; that is, the trips of one bus do not influence the trips of any other bus.

You are also given an integer totalTrips, which denotes the number of trips all buses should make in total. Return the minimum time required for all buses to complete at least totalTrips trips.



Example 1:

Input:
let time: Vec<i32> = vec![1, 2, 3];
let total_trips: i32 = 5;

Output: 3
Explanation:
- At time t = 1, the number of trips completed by each bus are [1,0,0].
  The total number of trips completed is 1 + 0 + 0 = 1.
- At time t = 2, the number of trips completed by each bus are [2,1,0].
  The total number of trips completed is 2 + 1 + 0 = 3.
- At time t = 3, the number of trips completed by each bus are [3,1,1].
  The total number of trips completed is 3 + 1 + 1 = 5.
So the minimum time needed for all buses to complete at least 5 trips is 3.
Example 2:

Input:
let time: Vec<i32> = vec![2];
let total_trips: i32 = 1;

Output: 2
Explanation:
There is only one bus, and it will complete its first trip at t = 2.
So the minimum time needed to complete 1 trip is 2.


Constraints:

1 <= time.length <= 105
1 <= time[i], totalTrips <= 107
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
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut hi: i64 = time
            .iter()
            .fold(0, |m, &t| m.max(t as i64 * total_trips as i64));
        let mut lo = 1;

        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::is_achived(&time, total_trips, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i64
    }

    fn is_achived(time: &[i32], tt: i32, c: i64) -> bool {
        let summ = time.iter().fold(0, |s, &t| {
            let temp = c / t as i64;
            s + temp
        });

        summ >= tt as i64
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {
        Solution::minimum_time;
        ex1: (vec![1, 2, 3], 5) => 3,
        ex2: (vec![2], 1) => 2,
    }
}
