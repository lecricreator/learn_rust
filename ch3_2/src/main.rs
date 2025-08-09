use std::io;

fn main() {
    let guess:i32 = "-442".parse().expect("Not a number");
    println!("Value is {guess}");
    let x = 2.0;
    let y:f64 = 3.000000;
    println!("x = {x} // y = {y}");
    let addition = 1.2 + 1.0;
    let substraction:i8 = 5 - 10;
    let multiplication:f64 = 4.0 * 121212111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111112121212121212121212111212121212121211278788778787788877878887878877878787778788878788778787878878877878787878787878787877878878787878787878787787878.98989899787878787745454545545544454544545544545;
    let division: f32 = -5.0 / -456.0;
    let remainder = -655 % 2;
    println!("add = {addition}, sub = {substraction}, multiple = {multiplication}, div = {division}, remainder = {remainder}");
    let bool = true;
    println!("le bool est {bool}");
    //char type
    let c = 'c';
    let b:char = 'C';
    let emoji = '😻';
    println!("char : {c}, {b}, {emoji}\n");
    //tuple type
    let mut tup: (i32, f32, i8) = (50000, 5.2, 125);
    let (x, y, z) = tup;
    println!("Discover tuple type {x} // {y} // {z}");
    tup.0 = 0;
    tup.1 -= 0.35;
    tup.2 %= 2;
    println!("modify tuple with period(.) : {2} // {0} // {1}", tup.0, tup.1, tup.2);
    println!("-----------------------------------------------------------------------------------");
    let mut array = [1, 0, 5, 4];
    array[1] += 5;
    println!("Array[0] : {0}\nArray[1] : {1}\nArray[2] : {2}\nArray[3] : {3}", array[0], array[1], array[2], array[3]);
    println!("----------------------------------------------------------------------------------");
    let array_same_value = [3; 4];
    println!("arraySameValue[0] : {0}\narraySameValue[1] : {1}\narraySameValue[2] : {2}\narraySameValue[3] : {3}", array_same_value[0], array_same_value[1], array_same_value[2], array_same_value[3]);
    println!("----------------------------------------------------------------------------------");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    println!("----------------------------------------------------------------------------------");
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);
}
