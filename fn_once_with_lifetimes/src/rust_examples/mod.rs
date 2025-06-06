/// This example comes from the Rust:
/// 
/// Please refer to https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
/// You can also try: rustc --explain E0499
///
/// The example is about borrow checking.
///
/// # Rust Borrow Checker Example
///
/// This example is adapted from the Rust Book:
/// - [The Rust Programming Language - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
/// - You can also try: `rustc --explain E0499`
///
/// The example demonstrates how Rust’s borrow checker enforces the rules of references
/// and lifetimes, particularly with mutable references.
///
/// ## Key Points:
/// - `foo` takes a mutable reference.
/// - `bar` also takes a mutable reference and then attempts to create an immutable borrow
///   (`let y = &a;`) while `foo` has a mutable borrow.
/// - This highlights Rust’s borrowing rules and helps understand the restrictions
///   around simultaneous mutable and immutable borrows.
///
/// Run this example by calling:
/// ```rust
/// rust_examples::borrow_example();
/// ```
///
/// Outputs:
/// ```text
/// Using the Rust example...
/// 2
/// ```



pub mod rust_examples {
    fn bar(_x: &mut i32){}

    fn foo(a: &mut i32){
        bar(a);
        let y = &a;
        println!("{}", y);
    }

    pub fn borrow_example(){
        let mut x = 2;
        println!("Using the Rust example...");

        foo(&mut x);
    }
}
