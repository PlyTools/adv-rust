//! Coryright (c) Liam 2023
//! All rights reserved under MIT license
//! 
//! A use case where you can apply borrowing rules to data 
//! which are not allowed to be mutately borrowed in compile time:


use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    {
        let mut mutable_reference = data.borrow_mut();
        *mutable_reference += 1;
    } // mutable_reference goes out of scope here, so the mutable borrow ends
    
    let immutable_reference = data.borrow();
    
    println!("data = {}", *immutable_reference);

}

