fn main() {
    
    println!("{}", fib(8));
    
}

fn fib(n: usize) -> i32 {
    let mut v: Vec<i32> = vec![0; n + 1]; 

    if n > 0 {
        v[1] = 1;
    }

    for i in 2..=n {
        v[i] = v[i - 1] + v[i - 2];
    }

    v[n]
}
