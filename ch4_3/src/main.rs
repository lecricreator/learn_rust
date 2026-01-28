fn main() {
    println!("1 string is : {}, 2 is : {}", return_str0(), return_str1());
}

fn return_str0() -> String {
    let str: String = "I'm a string !!!".to_string();
    str
}

fn return_str1() -> String {
    let str: String = String::from("I'm a second string");
    str
}
