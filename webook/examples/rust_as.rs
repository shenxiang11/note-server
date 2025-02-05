fn main() {
    let a = i64::MAX - 1;
    println!("a: {}", a);

    let b = a as i32;
    println!("b: {}", b);

    let c = -100;
    println!("c: {}", c);

    let d = c as u32;
    println!("d: {}", d);

    println!("{}", format!("{:0>6}", 1));
    println!("{}", format!("{:0>6}", 99999999));
}
