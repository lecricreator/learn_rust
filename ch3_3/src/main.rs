use std::i32;

fn main() {
    println!("IN Main!");
    first_func();
    third_function(455, "45tre");
    let mut value = return_function_five();
    println!("Cela n'a aucun sens que ce soit {value} et ecrit de cette façon.");
    value = add_func(value);
    println!("What's the logic of the return {value}");
}

fn first_func(){
    println!("first function !!");
    sec_fn_param(5);
}

fn sec_fn_param(x: i8){
    println!("second function param is : {}", x);
}

fn third_function(int_val: i32, char_val: &str){
    println!("Third function\nint_val : {int_val} // char_val : {char_val}");
}

fn return_function_five() -> i32{
    5
}

fn add_func(value: i32) -> i32{
    return value + 1;
}

