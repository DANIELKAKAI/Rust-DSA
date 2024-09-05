use std::collections::HashMap;

fn update_departments(s: String, map: &mut HashMap<String, Vec<String>>) {
    let mut index = 0;

    let mut user = String::new();
    let mut dep = String::new();

    for w in s.split_whitespace() {
        if index == 1 {
            user = w.to_string();
        }
        if index == 3 {
            dep = w.to_string();
            break;
        }
        index += 1;
    }

    // Only creates a new Vec if the key doesn't exist
    let users = map.entry(dep).or_insert_with(Vec::new);
    users.push(user);

    // Always creates a new Vec, even if the key exists
    //map.entry("key".to_string()).or_insert(vec![]);
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let s1 = String::from("Add Sally to Engineering");
    let s2 = String::from("Add Amir to Sales");
    let s3 = String::from("Add John to Sales");

    update_departments(s1, &mut map);
    update_departments(s2, &mut map);
    update_departments(s3, &mut map);

    println!("{:?}", map);
}
