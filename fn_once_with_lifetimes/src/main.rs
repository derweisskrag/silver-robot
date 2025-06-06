mod official_example;
mod my_examples;
mod rust_examples;


use official_example::example::example_closure;
use rust_examples::rust_examples::borrow_example;
use my_examples::my_examples::{
    use_closure_example_one, 
    use_closure_example_two, 
    use_closure_example_three
};


fn main() {
    // This code calls the function from `Official example`:
    example_closure(|list| &mut list[0]); // [4, 2, 3]
    example_closure(|list| &mut list[2]); // [1, 2, 4]


    // Try Rust example:
    borrow_example();

    // Try running the examples
    use_closure_example_one();
    use_closure_example_two();
    use_closure_example_three();
}
