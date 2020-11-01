use std::boxed::Box;

fn main() {
    box_simple();
}

/// BOX is a primitive pointer type
/// BOX allocate memory and assign value. 
/// BOX pointer is released automatically when it goes out of the scope  
fn box_simple() {
    // create a number typed pointer
    let mut int_pointer = Box::new(0);
    *int_pointer += 15;

    println!("{}", *int_pointer);

    // create a String typed pointer
    let mut sring_pointer = Box::new(String::from("Hello World !!"));
    println!("{}", *sring_pointer);

    *sring_pointer = String::from("Hello Another World !!");
    println!("{}", *sring_pointer);
}