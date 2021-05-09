/*
// Primitive types are
integeres: u - unsigned only positive, i - integer
8 16 32 64 128\
floats : f32, f64
bool
arrays, tuples
*/

pub fn run(){
    // def i32
    let x = 1;

    // default f64
    let y = 2.5;

    // add explicit types
    let y:i64 = 231312312;

    // find max size 
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i62: {}", std::i64::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u62: {}", std::u64::MAX);
}