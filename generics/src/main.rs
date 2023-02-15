//! Copyright (c) Liam 2022
//! All rights reserved. Licensed under MIT.
//!
//! The examples in Chapter 10.1
//! 

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn using_generic_function() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn using_generic_struct() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y : 4.0};

    let integer_and_float = MixedPoint {x: 5, y: 4.0};

}


fn main() {
    // using_generic_function();
    using_generic_struct();

}