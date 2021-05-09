pub fn run(){
    let person: (&str, &str, i8) = ("Brad","Mass",35);
    println!("{} from {} and is {}", person.0, person.1, person.2)
}