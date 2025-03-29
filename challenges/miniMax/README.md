# Mini-max sum problem (easy)

## Table of Contents

- [Objective](#objective)
- [Solution](#solution)
- [Rust](#rust)
    - [Using fold](#using-fold)
- [Python](#python)
- [C#](#c)
- [Kotlin](#kotlin)
- [JavaScript](#javascript)
- [C++](#c-1)

## Objective

To get insight into the programming languages that I study: what basic functionality
they offer me?

## Solution

This particular problem is very easy: it requires us to think mathematically. Yes, I will
try to solve this later on without using math, but I want to train my brain
to use math too. In fact, one of LeetCode problems wanted us to remember

```
S_n = (n * (n + 1)) / 2
```
where S means `Sum` for the arithmetic progression `a_1, a_2, ..., a_n`.
By the way, if you set `n` to approach the infinity, then `S = infinity`, right?
In programming, we work with finite numbers, rather than infinite - advanced math.

So, in order to solve the mini-max sum challenge,
we have to compute the sum as we do normally, but then, we notice that if we
subtract the minimum or maximum element from this sum, we get those
mini and max sums, as required. It is subtle!

## Rust

Please, do not criticize me for using `for loop` twice, I will study this today.

```rust
fn mini_max_sum(nums: &[i32]) {
    let maximum_number = nums.iter().max().expect("The array must not be empty!");
    let minimum_number = nums.iter().min().expect("The array must not be empty!");
    let array_sum = nums.iter().sum();
    
    // now we can compute the minimum and maximum sums
    let minimum_sum = array_sum - minimum_number;
    let maximum_sum = array_sum - maximum_number;
    
    println!("The results are computed!\n\tThe maximum sum is {maximum_sum}\n\tThe minimum sum is {minimum_sum}");
}
```

### Using fold

```rust
fn mini_max_sum(nums: &[i32]) {
    let (min, max, sum) = nums.iter().fold(
        (i32::MAX, i32::MIN, 0),
        |(min, max, sum), &num| (min.min(num), max.max(num), sum + num),
    );

    let minimum_sum = sum - max;
    let maximum_sum = sum - min;

    println!("The results are computed!\n\tThe maximum sum is {maximum_sum}\n\tThe minimum sum is {minimum_sum}");
}
```

## Python

```python
def mini_max_sum(nums: list[int]):
    min_number, max_number, total_sum = float('inf'), float('-inf'), 0
    for num in nums:
        total_sum += num
        if num < min_number:
            min_number = num
        elif num > max_number:
            max_number = num
            
    minimum_sum = total_sum - min_number
    maximum_sum = total_sum - max_number
    print(f"The results are computed!\n\tThe maximum sum is {maximum_sum}\n\tThe minimum sum is {minimum_sum}")
```

Or in one line using `sum`, `min`, `max`:

```python
def mini_max_sum(nums: list[int]):
    print(f"The maximum sum is {sum(nums) - min(nums)}\nThe minimum sum is {sum(nums) - max(nums)}")
```

## C#

```csharp
namespace App {
    public class Program {
        public static void Main(string[] args){
            // create mock array
            int[] InputArray = new int[] {1, 2, 3, 4, 5};
            (int MaximumSum, int MinimumSum) = MiniMaxSum(InputArray);
            Console.WriteLine($"Maximum Sum: {MaximumSum}, Minimum Sum: {MinimumSum}");
        }
        
        public static (int, int) MiniMaxSum(int[] nums){
            // defining the variables:
            // max, min, sum
            // Arrays usually don't have helpers methods
            // I remember that List<int> does have it for us.
            List<int> NumsList = nums.ToList();
            
            if (nums == null || nums.Length == 0)
                throw new ArgumentException("The array must not be null or empty.");
            
            // Once we converted it, we can try methods
            int MaximumElement = NumsList.Max();
            int MinimumElement = NumsList.Min();
            int NumsSum = NumsList.Sum();
            
            return (NumsSum - MinimumElement, NumsSum - MaximumElement);
        }
    }
}
```

## Kotlin

```kotlin
fun miniMaxSum(nums: Array<Int>): Pair<Int, Int>{
    val minimumNumber = nums.minOrNull() ?: throw IllegalArgumentException("Array must not be empty")
    val maximumNumber = nums.maxOrNull() ?: throw IllegalArgumentException("Array must not be empty")
    val numsSum = nums.sum()
    
    val minimumSum = numsSum - maximumNumber
    val maximumSum = numsSum - minimumNumber

    return Pair(minimumSum, maximumSum)
}
```

## JavaScript

```ts
function miniMaxSum(nums: number[]) {
    let maximumNumber = nums.reduce((curr, acc) => curr > acc ? curr : acc, -Infinity);
    let minimumNumber = nums.reduce((curr, acc) => curr < acc ? curr : acc, Infinity);
    let totalSum = nums.reduce((acc, curr) => acc + curr, 0);
    let minimumSum = totalSum - maximumNumber;
    let maximumSum = totalSum - minimumNumber;
    console.log(`The results are computed!`);
    console.log(`\tThe maximum sum is ${maximumSum}`);
    console.log(`\tThe minimum sum is ${minimumSum}`);
}
```

## C++

```cpp
#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

pair<int, int> miniMaxSum(const std::vector<int>& nums){
    int maximumNumber = *std::max_element(nums.begin(), nums.end());
    int minimumNumber = *std::min_element(nums.begin(), nums.end());
    int totalSum = std::accumulate(nums.begin(), nums.end(), 0);
    int minimumSum = totalSum - maximumNumber;
    int maximumSum = totalSum - minimumNumber;
    return { minimumSum, maximumSum };
}
```

