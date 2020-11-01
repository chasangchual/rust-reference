use std::boxed::Box;
use std::rc::Rc;

fn main() {
    box_simple();
    rc_simple();
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

fn rc_simple() {
    let int_pointer = Rc::new(55);
    println!("rc value: {}", *int_pointer);
    rc_simple_get_reference_count(&int_pointer);

    let int_pointer_clone1 = int_pointer.clone();
    println!("rc value: {}", *int_pointer_clone1);
    rc_simple_get_reference_count(&int_pointer);
    
    let int_pointer_clone2 = int_pointer.clone();
    println!("rc value: {}", *int_pointer_clone2);
    rc_simple_get_reference_count(&int_pointer);

    rc_simple_get_reference_count(&int_pointer);
    rc_simple_get_reference_count(&int_pointer_clone1);
    rc_simple_get_reference_count(&int_pointer_clone2);

    rc_simple_pointer_value(&int_pointer);

    rc_simple_pointer(&int_pointer);
    rc_simple_pointer(&int_pointer_clone1);
    rc_simple_pointer(&int_pointer_clone2);
}

fn rc_simple_get_reference_count<T>(ptr:&Rc<T>) {
    let reference_count = Rc::strong_count(ptr);
    println!("rc reference count: {}", reference_count);
}

fn rc_simple_pointer_value<T: std::fmt::Display>(ptr:&Rc<T>) {
    println!("rc pointer count: {}", ptr);

    let t = ptr;
    println!("t: {}", t);
}

fn rc_simple_pointer<T>(ptr:&Rc<T>) {
    println!("rc pointer count: {:?}", Rc::as_ptr(ptr));
}
