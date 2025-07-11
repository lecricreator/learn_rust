fn main() {
    let mut x = 5;
    println!("value x is {x}");
    x = 6;
    println!("value x is {x}");
    //banger les shodows
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //utiliser le même nom de variable pour différente donné et type
    let space = "   ";
    let space = space.len();
    println!("nbr space {space}");
}
