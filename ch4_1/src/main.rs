//learn gestionary of heap
fn main() {
    let infinite = Box::new([100; 1000_000]);
    let mut s = String::from("test");
    let copy;
    println!("{:?}", infinite);
    let rob = infinite;
    println!("{:?}", rob);
    s.push_str(" of test 0");
    
    copy = s;
    //after s cannot be used
    println!("{} / test1", copy);
}
