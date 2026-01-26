//learn gestionary of heap
fn main() {
    let mut s = String::from("test");
    let copy;
    s.push_str(" of test 0");
    copy = s;
    //after s cannot be used
    println!("{} / test1", s);
}
