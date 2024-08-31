
fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    
    let left_len = left.len();
    let right_len = right.len();
    
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    
    let mut final_vec = vec![];
    
    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            final_vec.push(left[i]);
            i += 1;
        }
        else{
            final_vec.push(right[j]);
            j += 1;
        }
    };
    
    while i < left_len {
        final_vec.push(left[i]);
        i+=1
    };
    
    while j < right_len {
        final_vec.push(right[j]);
        j +=1
    };
    
    final_vec
}


fn merge_sort(list: Vec<i32>) -> Vec<i32>{
    if list.len() <= 1 {
        return list;
    };
    
    let mid = list.len() / 2;
    
    let left = &list[0..mid];
    let right = &list[mid..];
    
    let left_merge = merge_sort(left.to_vec());
    let right_merge = merge_sort(right.to_vec());
    
    let sorted = merge(left_merge, right_merge);
    
    sorted
}


fn main() {
    
    let list = merge_sort(vec![33,5,1,20,33,24,3,8,1]);
    
    println!("{:?}", list);
    
}



