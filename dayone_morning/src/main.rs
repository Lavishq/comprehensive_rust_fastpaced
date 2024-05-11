
// Exercise: Fibonacci

// The Fibonacci sequence begins with [0,1]. For n>1, the n’th Fibonacci number is calculated recursively as the sum of the n-1’th and n-2’th Fibonacci numbers.

// Write a function fib(n) that calculates the n’th Fibonacci number. When will this function panic?

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        // todo!("Implement this")
        n
    } else {
        // The recursive case.
        // todo!("Implement this")
        fib(n-1) + fib(n-2)
    }
}

mod gcd;
// use crate::gcd::gcd::find_gcd;
use crate::gcd::gcd::{find_gcd, factorial};


/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: u32) -> u32 {
    if n == 1 {
        println!("{}",n);
        return 1;
    } else if n % 2 == 0 {
        println!("{}",n);
        collatz_length(n/2)
    } else {
        println!("{}",n);
        collatz_length((n*3)+1)
    }
}

fn main() {

    // exercise 1 

    let n = 20;
    // println!("gcd: {}", find_gcd(52, 143));
    // println!("gcd: {}", find_gcd(143, 52));
    // todo!();
    
    println!("fib({n}) = {}", fib(n));

    // exercise 1 end

    // exercise 2
    collatz_length(3);

    // exercize 2 end

    let n = 4;
    println!("{n}! = {}", factorial(n));
}

// solution by https://google.github.io/comprehensive-rust/control-flow-basics/solution.html

// /// Determine the length of the collatz sequence beginning at `n`.
// fn collatz_length(mut n: i32) -> u32 {
//     let mut len = 1;
//     while n > 1 {
//         n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
//         len += 1;
//     }
//     len
// }

// #[test]
// fn test_collatz_length() {
//     assert_eq!(collatz_length(11), 15);
// }

// fn main() {
//     println!("Length: {}", collatz_length(3));
// }