pub fn run(){
    let mut hello = String::from("Hello ");

    println!("Length: {}",hello.len());
    hello.push('W');
    hello.push_str("orld");
    println!("{}",hello);


    // capacity in bytes
    println!("cap {}",hello.capacity());
    println!("empty {}",hello.is_empty());
    println!("caontaint world {}",hello.contains("world"));
}