use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let diff = target - value;

        match map.get(&value) {
            Some(&v) => {
                return vec![v, index as i32];
            }
            _ => {
                map.insert(diff, index as i32);
            }
        }
    }

    println!("{:?}", map);

    vec![]
}

fn main() {
    let res = two_sum(vec![2, 7, 11, 15], 9);

    println!("{:?}", res);
}
