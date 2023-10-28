use sdk;
// use secret;

fn main() {
    let a = sdk::add(2, 5);
    println!("{0}", a);
    println!("Hello, world!");
    let b = sdk::my_module::div(1, 3).unwrap();
    println!("the value of {0}, is {1}", stringify!(b), b);
}
