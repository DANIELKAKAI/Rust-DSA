
fn quick_sort(mut items: Vec<i32>) -> Vec<i32> {

    if items.len() <= 1 {
        return items;
    }
    
    let mid = items.len() / 2;
    
    let pivot = items.remove(mid);
    
    let mut left = vec![];
    let mut right = vec![];
    
    // iter returns ref
    // into_iter returns value
    for (i, item) in items.into_iter().enumerate() {
        if i != mid {
            if item >= pivot {
                right.push(item);
            }
            else{
                left.push(item);
            }
        }
    }
    
    let mut left_sort = quick_sort(left);
    let mut right_sort = quick_sort(right);
    
    left_sort.push(pivot);
    
    left_sort.append(&mut right_sort);
    
    left_sort
}

fn main() {
    
    let items = vec![1,2,22,8,3,4,5,9,0];
    
    let sorted_items = quick_sort(items);
    
    println!("{:?}", sorted_items);
    
}
