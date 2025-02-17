fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];

    let mut prev_cap = arr.capacity();
    for i in 1..i32::max_value() {
        arr.push(i);
        let current_cap = arr.capacity();
        if current_cap != prev_cap {
            println!("capicity: {}", current_cap);
            prev_cap = current_cap;
        }
    }
}

// rust vec 的扩容就是 2 倍扩容
