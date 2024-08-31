
fn binary_search(array: &[i32;5], target: i32) -> bool {
    let mut mid : usize;
    let mut temp_array = &array[..];
    
    while temp_array.len() > 0 {
        mid = temp_array.len() / 2;
        if target == temp_array[mid] {
            return true;
        }
        if temp_array.len() == 1 {
            break;
        }
        if target <= temp_array[mid] {
            temp_array = &temp_array[0..mid];
        }
        else{
            temp_array = &temp_array[mid..];
        }
    }
    
    false
}


fn main() {
  
    let array : [i32;5] = [1,2,3,4,5];
  
    let res = binary_search(&array, 3);
    
    println!("{}", res);
    
}



