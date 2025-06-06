/// # Example Module
///
/// This module demonstrates how to use **closures with lifetimes** in Rust,
/// especially in the context of borrowing and mutating slices.
///
///
/// ## Source:
///
/// You can find the code at the link: https://github.com/rust-lang/rust/issues/29625
/// To locate the code snippet, please, read until you hit the bottom (the last message).
///
///
///## Author
///
/// You find their GitHub profile: https://github.com/bcmpinc
///
///
/// ## Functions:
///
/// - `example_closure`:
///     - Accepts a closure with a **lifetime parameter** `'a`, ensuring that
///       the borrowed slice and the returned mutable reference have the same lifetime.
///     - Demonstrates how to pass a closure that **takes a mutable slice**
///       and returns a **mutable reference to an element** inside that slice.
///     - Inside the function:
///         - Initializes an array `[1, 2, 3]`.
///         - Applies the closure to mutate one of the elements.
///         - Prints the modified array.
///
/// ## Example Usage:
///
/// ```rust
/// example::example_closure(|list| &mut list[0]); // [4, 2, 3]
/// example::example_closure(|list| &mut list[2]); // [1, 2, 4]
/// ```
///
/// ## Notes:
/// - This example is inspired by Rust’s ownership and borrowing concepts,
///   demonstrating how **lifetime annotations** enable safe borrowing with closures.
/// - Useful for learning **borrowing**, **mutability**, and **closure signatures with lifetimes** in Rust.
///
/// ---



pub mod example {
    pub fn example_closure(closure_with_lifetime: impl for<'a> Fn(&'a mut [i32]) -> &'a mut i32) {
        let mut x = [1, 2, 3];
        *closure_with_lifetime(&mut x) = 4;
        println!("{:?}", x);
    } 
}
