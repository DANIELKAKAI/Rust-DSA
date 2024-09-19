use std::cmp::{max};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;
        let mut max_p = 0;

        while right < prices.len() {
            if &prices[left] < &prices[right] {
                let profit = &prices[right] - &prices[left];
                max_p = max(max_p, profit);
            }
            else {
                left = right
            }

            right += 1;
        }

        max_p
    }
}
