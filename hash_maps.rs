use std::collections::HashMap;

fn main() {
    
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    map.insert(1,1);
    map.insert(2,2);
    
    match map.get(&1) {
        Some(v) => println!("{v}"),
        _ => println!("z")
    };
    
    map.entry(3).or_insert(3);
    
    let x = map.get(&2).unwrap(); //immutable ref
    
    let s = map.get(&4).copied().unwrap_or(4);
    
    println!("{} {}", x, s);
    
    println!("{:?}", map);
}
