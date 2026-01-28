fn main() {
    learn_both_return();
    learn_references();
}

fn learn_both_return(){
    println!("----------------------------------------------------------------\nBoth return");
    let start = String::from("Helloy /");
    let next = String::from("worldo !!!");
    let (start1, next1) = greet_return(start, next);
    let test = format!("{} {}", start1, next1);
    println!("{}", test);
}

fn learn_references(){
    println!("----------------------------------------------------------------\nlearn references");
    let start = String::from("Helloy /");
    let next = String::from("worldo !!!");
    greet_references(&start, &next);
    let test = format!("{} {}", start, next);
    println!("{}", test);

}

fn greet_return(p_start: String, p_end: String) -> (String, String){
    println!("{} {}", p_start, p_end);
    (p_start, p_end)
}

fn greet_references(p_start: &String, p_end: &String){
    println!("{} {}", p_start, p_end);
}