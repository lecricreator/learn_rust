fn main() {
    let total_string = String::from("C'est un grand jour.");
    println!("Size is {}", exercise_seperate(&total_string));
    string_slice();
    let progression_str: &str;
    let mut i_progress = 0;
    loop {
        if i_progress < total_string.len() {
            (progression_str, i_progress) = return_first_word(i_progress, &total_string);
            println!("{}", progression_str);
        }
        {
            break;
        }
    }
}

fn string_slice() {
    let str_hello = String::from("Hello world");
    let first_space = exercise_seperate(&str_hello);
    let first_word: &str = &str_hello[0..first_space];
    let end_word: &str = &str_hello[first_space + 1..str_hello.len()];
    println!("{} {}", first_word, end_word);
}

fn exercise_seperate(total_string: &String) -> usize {
    let bytes = total_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    total_string.len()
}

fn return_first_word(start_word: usize, total_string: &String) -> (&str, usize) {
    let slice = &total_string[start_word..];

    for (i, c) in slice.char_indices() {
        if c == ' ' {
            return (&total_string[start_word..i], i);
        }
    }

    (&total_string[start_word..], total_string.len())
}
