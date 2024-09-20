use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut values : HashSet<i32> = HashSet::new();

        for n in nums{
            if values.contains(&n) {
                return true;
            }
            values.insert(n);
        }
        return false;
    }
}
