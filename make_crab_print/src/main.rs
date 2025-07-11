use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");
        let pomme = rand::thread_rng().gen_range(1..=2);
        println!("secret number is : {pomme}");
        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_int.cmp(&pomme) {
            Ordering::Less => print!("Too small\n"),
            Ordering::Greater => print!("Too bigger\n"),
            Ordering::Equal => {
                println!("equal");
                break ;
            }
        }
        guess.clear();
    }
}
