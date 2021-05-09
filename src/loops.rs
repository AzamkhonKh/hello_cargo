pub fn run(){
    let mut count = 0;

    // infinite loops
    loop {
        count += 1;
        println!("count = {}",count);

        if count == 10 { break; }
    }
}