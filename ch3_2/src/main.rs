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
}
