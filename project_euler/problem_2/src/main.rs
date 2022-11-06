

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.
use std::time::{Instant};

fn main() {

    let now = Instant::now();

    let mut sum: i32 = 2;
    let mut fib1: i32 = 1;
    let mut fib2: i32 = 2;
    let mut temp: i32;
    while fib1+fib2 < 4000000 {
        temp = fib1 + fib2;
        if temp % 2 == 0 {
            sum = sum + temp;
        }
        fib1 = fib2;
        fib2 = temp;
    }
    println!("{}",sum);
    println!("{}", now.elapsed().as_micros());
}
