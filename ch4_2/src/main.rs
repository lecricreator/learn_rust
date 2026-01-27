fn main() {
    let start = String::from("Helloy /");
    let next = String::from("worldo !!!");
    let (start1, next1) = greet(start, next);
    let test = format!("{} {}", start1, next1);
    println!("{}", test);
}

fn greet(p_start: String, p_end: String) -> (String, String){
    println!("{} {}", p_start, p_end);
    (p_start, p_end)
}