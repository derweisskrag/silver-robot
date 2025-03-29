# Process Array

## Table of Contents
- [Description](#description)
- [Objective](#objective)
- [Solution](#solution)
- [JavaScript Implementations](#js)
    - [Iterative Approach](#iterative-approach)
    - [Recursive Approach](#recursive-approach)
    - [Processing Nested Arrays](#processing-nested-arrays)

## Description

Processing an array can be done iteratively or recursively. This document explores both approaches, highlighting key learning moments from 2022, post-military service.

## Objective

The goal is to understand different ways to traverse and manipulate arrays using recursion and iteration. We also highlight pitfalls in function parameter ordering.

## Solution

To ensure correct function execution, default parameters must be placed at the end. For example:

```js
function foo(arg1, k=0, arg2, arg3){
    /* some logic */
}
```

This causes issues when calling:

```js
foo("Hello", "Bro?", "Ayo");
```

Here, `k = "Bro?"` instead of being `0`, and `arg3` is `undefined`. The fix is:

```js
function foo(arg1, arg2, arg3, k = 0){
    /* some logic */
}
```

This ensures arguments align correctly when the function is invoked.

## JavaScript Implementations

### Iterative Approach

This method extends `Array.prototype` to implement a custom `reduce` function.

```js
Array.prototype.MyReduce = function(callback, initialValue = 0) {
    for (let i = 0; i < this.length; i++) {
        initialValue = callback.apply(null, [this[i], initialValue]);
    }
    return initialValue;
};
```

- Uses a `for` loop to iterate over the array.
- Calls the provided callback function on each element.
- Accumulates and returns the final computed value.

### Recursive Approach

A recursive implementation of array processing with custom logic.

```js
function processArray(arr, callback, initial = arr[0], n = 0) {
    if (arr[n] < 0) {
        arr[n] = 0; // Replace negative numbers with 0
    }
    if (n >= arr.length) {
        return initial; // Base case: end of array
    }
    return processArray(arr, callback, callback(arr[n], initial), n + 1);
}
```

- Handles negative values by setting them to `0`.
- Uses recursion to process each element.
- Base case ensures termination when the end of the array is reached.

### Processing Nested Arrays

This function flattens an array of objects and accumulates values based on the `name` property.

```js
function processArray(...arr) {
    const data = arr.flat();
    const tally = data.reduce((acc, item) => {
        const { name, ...points } = item;
        acc[name] = acc[name] || {};
        Object.entries(points).forEach(([key, val]) => {
            acc[name][key] = (acc[name][key] || 0) + val;
        });
        return acc;
    }, {});
    return tally;
}
```

- Uses the spread operator (`...arr`) to accept multiple arrays.
- Flattens the input arrays into a single array.
- Uses `reduce` to group and sum properties based on the `name` key.
- Ensures undefined values are initialized to `0` to avoid `NaN` results.

---

This structure makes it easier to understand, modify, and expand upon these functions. Let me know if you need any refinements! 🚀

## Kotlin

## Rust

## C#

## C++ 

## Python