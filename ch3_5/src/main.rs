fn main() {
    let bool:bool = true;
    if bool {
        println!("true");
    }
    else {
        println!("false");
    }
    let numb:i32 = 5;
    if numb < 5 {
        println!("valeur inf à 5");
    }
    else {
        println!("valeur sup ou égale à 5");
    }
    let mut result = if bool {numb + 5} else {numb + 10};
    println!("result is {}", result);

    let mut counter = 0;
    result = loop {
        counter += 1;
        if counter == 5{
            break counter * 2;
        }
        println!("MOVE + 1")
    };
    println!("The result is {result}");
    println!("-------------------------------------------------------------------");
    let mut count = 0;
    'loop_stopable: loop{
        println!("counter = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 8{
                break;
            }else if count == 3{
                break 'loop_stopable;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("-------------------------------------------------------------------");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Sort de la boucle.");
    let a:[i64; 4] = [5, 10, 15, 100];
        println!("GOOD PARCTICE-----------------------------------------------------");
    for value in a {
        println!("the value is {value}");
    }
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("RISK OF PANICK-----------------------------------------------------");
    let mut i: usize = 0;
    while a[i] < 400 {
        println!("TEST : {}", a[i]);
        i += 1;
    }
}
