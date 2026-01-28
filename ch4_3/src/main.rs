fn main() {
    println!("1 string is : {}, 2 is : {}", return_str0(), return_str1());
    //when you not enough permission
    let mut str_to_complete: Vec<String> = vec![];
    let str: String = String::from("My name is ");
    str_to_complete.push(str);
    str_to_complete.push(String::from("jean sartre."));
    let result1 = complete_end_string(&mut str_to_complete);
    str_to_complete.push(result1);
    let result2 = complete_end_string1(str_to_complete);

    println!("{}", result2);
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
    start_str.push(String::from(" I'm a lawyer.\n"));
    let full = start_str.join("");
    full
}

fn stringify_name_with_title0(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

fn complete_end_string1(start_str: Vec<String>) -> String {
    let mut start_str_clone: Vec<String> = start_str.clone();
    start_str_clone.push(String::from("You're guilty !!!"));
    let total_sting = start_str_clone.join("");
    total_sting
}
