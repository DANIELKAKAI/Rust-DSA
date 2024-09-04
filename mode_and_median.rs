use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    mode: i32,
    median: i32,
}

fn mode_and_median(list: &mut Vec<i32>) -> Stats {
    list.sort();

    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut max: i32 = 0;
    let mut max_freq = 0;

    for &item in list.iter() {
        let frequency = map.get(&item).copied().unwrap_or(0);
        map.insert(item, frequency + 1);

        if frequency > max_freq {
            max = item;
            max_freq = frequency;
        }
    }

    println!("sorted list is {:?}", list);

    let size = list.len();

    let mid_index = size / 2;

    let mut median: i32 = list[mid_index];

    if mid_index % 2 == 0 {
        median = (list[mid_index] + list[mid_index - 1]) / 2
    }

    Stats {
        median: median,
        mode: max,
    }
}

fn main() {
    let mut v = vec![11, 2, 31, 2, 3, 3, 3, 4, 5, 3];

    let x = mode_and_median(&mut v);

    println!("{:?}", x);
}
