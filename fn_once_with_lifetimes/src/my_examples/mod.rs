/// # My Examples Module
///
/// This module demonstrates various ways to work with **closures** in Rust,
/// including:
///
/// - Using closures as **function parameters** with `FnOnce`.
/// - Modifying **vectors and slices** via closures.
/// - Demonstrating **lifetime annotations** in closures that mutate data.
///
/// ## Functions:
///
/// - `closure_example_one`:
///     - Accepts an `i32` value and a closure that takes ownership (`FnOnce`) and returns an `i32`.
///     - Applies the closure to the input and returns the result.
///     - Example usage:
///       ```rust
///       let add_one = |x: i32| x + 1;
///       let result = closure_example_one(5, add_one); // result = 6
///       ```
///
/// - `closure_example_two`:
///     - Accepts a `Vec<i32>` and a closure that takes a mutable slice of the vector (`&mut [i32]`).
///     - Modifies the slice in place.
///     - Returns the modified vector.
///     - Demonstrates using **for<'a>** lifetime bounds to allow the closure to borrow the slice.
///     - Example usage:
///       ```rust
///       let result = closure_example_two(vec![1, 2, 3], |x| x[2] = 4);
///       println!("{:?}", result); // [1, 2, 4]
///       ```
///
/// - `closure_example_three`:
///     - Accepts a mutable slice, an index, and a closure that modifies a single element.
///     - Returns a mutable reference to the modified element.
///     - Uses explicit **lifetime annotations** (`'a` and `'b`) to ensure safe borrowing.
///     - Example usage:
///       ```rust
///       let mut data = vec![1, 2, 3];
///       let modified = closure_example_three(&mut data, 1, |val| *val = 99);
///       println!("Modified: {}", modified); // 99
///       ```
///
/// ## Usage Examples:
///
/// - `use_closure_example_one`: Demonstrates usage of `closure_example_one` with simple addition closures.
/// - `use_closure_example_two`: Demonstrates modifying elements in a vector using closures.
/// - `use_closure_example_three`: Demonstrates modifying an element at a specific index with a closure.
///
/// ## Notes:
/// - These examples illustrate **closure capturing** and **lifetime management** in Rust.
/// - Useful for learning how closures can be used in different scenarios involving ownership and mutability.
///
/// ---



pub mod my_examples {
    fn closure_example_one<F: FnOnce(i32) -> i32>(x: i32, callback: F) -> i32 {
        // applies the callback on the input `x`
        // returns the result
        callback(x)
    }


    fn closure_example_two<F>(mut v: Vec<i32>, f: F) -> Vec<i32> 
    where
        F: for<'a> FnOnce(&'a mut [i32])
    {
        f(&mut v);
        v
    }


    fn closure_example_three<'a, F>(data: &'a mut [i32], index: usize, mutator: F) -> &'a mut i32 
    where
        F: for<'b> FnOnce(&'b mut i32)
    {
        let val_ref = &mut data[index];
        mutator(val_ref);
        val_ref
    }


    pub fn use_closure_example_one(){
        let add_one = |x: i32| {
            x + 1
        };

        let add_two = |x: i32| {

            x + 2
        };


        println!("Calling closure_example_one with `x` = 5 and `callback` = add_one: {}", closure_example_one(5, add_one));
        println!("Calling closure_example_one with `x` = 10 and `callback` = add_two: {}", closure_example_one(10, add_two));
    }


    pub fn use_closure_example_two(){
        let v = closure_example_two(vec![1, 2, 3], |x| x[2] = 4);
        println!("Calling closure_example_two with `vec` = vec![1, 2, 3] and `callback` = `x[2] = 4`: {:?}", v);
        println!("Calling closure_example_two with `vec` = vec![7, 1, 2, 4, 5] and `callback` = `x[4] = 100`: {:?}", closure_example_two(vec![7, 1, 2, 4, 5], |x| x[4] = 100));

    }


    pub fn use_closure_example_three(){
        println!("You are using the `use_closure_example_three`...");

            let mut numbers = vec![1, 2, 3, 4, 5];
            let idx = 2;

            let closure = |val_ref: &mut i32| {
                *val_ref = 42; // Dereference and mutate
            };

            let modified_ref = closure_example_three(&mut numbers, idx, closure);

            println!("Modified value: {}", modified_ref); // Prints: Modified value: 42
            println!("Numbers: {:?}", numbers);         // Prints: Numbers: [1, 2, 42, 4, 5]
    }

}
