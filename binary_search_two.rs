
fn binary_search(array: &[i32;5], target: i32) -> bool {
    let mut start = 0 as usize;
    let mut end = array.len() as usize;
    let mut mid : usize;
    
    while start < end {
        mid = (start + end) / 2;
        
        if target == array[mid] {
            return true;
        }
        
        if target <= array[mid] {
            end = mid;
        }
        else{
            start = mid + 1;
        }
    }
    
    false
}


fn main() {

    let array : [i32;5] = [1,2,3,4,5];
    
    let res = binary_search(&array, -1);
    
    println!("{}", res);
    
}



