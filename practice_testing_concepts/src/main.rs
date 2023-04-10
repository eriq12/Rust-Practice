pub mod fib;
use crate::fib::fib;

fn main() {
    let n : u32 = 14;
    let fibn : u32 = fib(n);
    println!("The {n} number of fib sequence is {fibn}.");
}
