//! Copyright (c) Liam 2023
//! All rights reserved under MIT license
//! 
//! Examples of smart pointer Box<T> in Chapter 15.1

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
