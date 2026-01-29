fn main() {
    println!("1 string is : {}, 2 is : {}", return_str0(), return_str1());
    let mut test: Vec<String> = vec!["Test".to_string()];
    //modifie the vector inside function and can be used after the end of fn
    let value_string0: String = complete_end_string(&mut test);
    //modify the value inside but the variable is deallocated.
    //Cannot be used after reading the function.
    let value_string1: String = stringify_name_with_title(test);
    println!("{} / {}", value_string0, value_string1);
    //error : test has modification and change place of allocation memory.
    //test.push(String::from("Adapt."));
}

fn return_str0() -> String {
    let str: String = "I'm a string !!!".to_string();
    str
}

fn return_str1() -> String {
    let str: String = String::from("I'm a second string");
    str
}

fn complete_end_string(start_str: &mut Vec<String>) -> String {
    start_str.push(String::from(" I'm a lawyer."));
    let total_str = start_str.join("");
    total_str
}

fn stringify_name_with_title(mut start_str: Vec<String>) -> String {
    start_str.push(String::from("I'm guilty."));
    let total_str = start_str.join(" ");
    total_str
}
