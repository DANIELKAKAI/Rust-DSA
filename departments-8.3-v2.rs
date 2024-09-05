use std::collections::HashMap;

fn update_departments(s: &str, map: &mut HashMap<String, Vec<String>>) {
    let mut words = s.split_whitespace();
    let user = words.nth(1).unwrap().to_string();
    let dep = words.nth(1).unwrap().to_string();

    let users = map.entry(dep).or_insert_with(Vec::new);
    users.push(user);
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let strings = [
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "Add John to Sales",
    ];

    for s in strings {
        update_departments(s, &mut map);
    }

    println!("{:?}", map);
}
