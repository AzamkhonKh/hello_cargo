use std::mem;
pub fn run(){
    let mut  numbers: Vec<i32> = vec![1,1,2,34,5];
    println!("{:?}", numbers);
    println!("index 2 before {}", numbers[2]);
    numbers[2] = 3;
    println!("index 2 after {}", numbers[2]);
    println!("Vector len  = {}", numbers.len());
    println!("Value numbers {} bytes", mem::size_of_val(&numbers));
    println!("{:?}", numbers);

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}