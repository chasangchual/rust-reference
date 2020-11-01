use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub fn rc_simple() {
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

    rc_simple_get_reference_count(&int_pointer);

}

fn rc_simple_get_reference_count<T>(ptr:&Rc<T>) {
    let reference_count = Rc::strong_count(ptr);
    println!("rc reference count: {}", reference_count);
}

fn rc_simple_pointer_value<T: std::fmt::Display>(ptr:&Rc<T>) {
    println!("rc pointer count: {}", ptr);

    let t = ptr.clone();
    println!("t: {}", t);

    rc_simple_get_reference_count(&ptr);
    rc_simple_get_reference_count(&t);
}

fn rc_simple_pointer<T>(ptr:&Rc<T>) {
    println!("rc pointer count: {:?}", Rc::as_ptr(ptr));
}




pub fn rc_cell_simple() {
    let int_pointer = Rc::new(RefCell::new(55));
    
    let val2: Ref<_> = int_pointer.borrow();
    println!("rc_cell_simple {}", val2);

    let mut val1: RefMut<_> = int_pointer.borrow_mut();
    println!("rc_cell_simple {}", val1);

    *val1 = *val1 + 100;
    println!("rc_cell_simple {}", val1);

}