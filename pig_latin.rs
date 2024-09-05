
fn pig_latin(mut s:String) -> String {
    let vowels = ['a','e', 'i', 'o', 'u'];
    
    let first_char = s.chars().next().unwrap();
    
    let f_lower = first_char.to_lowercase().next().unwrap();
    
    if vowels.contains(&f_lower) {
        s.push_str("-hay");
    }
    else{
        s.push_str("ay");
    }
    
    s
}

fn main() {
    let my_string = String::from("world");
    
    let res = pig_latin(my_string);
    
    println!("{}", res);
    
    let my_string2 = String::from("Apex");
    
    let res2 = pig_latin(my_string2);
    
    println!("{}", res2);
}
