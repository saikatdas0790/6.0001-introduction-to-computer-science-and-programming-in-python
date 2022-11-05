// Assignment:
//  Write a program that does the following in order:
//  1. Asks the user to enter a number “x”
//  2. Asks the user to enter a number “y”
//  3. Prints out number “x”, raised to the power “y”.
//  4. Prints out the log (base 2) of “x”.

use std::io;

fn main() {
    println!("Enter a number x: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: f64 = x.trim().parse().expect("Please type a number!");

    println!("Enter a number y: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y: f64 = y.trim().parse().expect("Please type a number!");

    println!("x^y = {}", x.powf(y));
    println!("log(x) = {}", x.log2());
}
