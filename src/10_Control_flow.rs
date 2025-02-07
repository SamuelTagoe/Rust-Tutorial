// If Expressions
// Let's you branch your code based on certain condition

#![allow(warnings)]
fn main() {
    let age:u16 = 18;

    if age >= 18 {
        println!("You are old enough to drive");
    } else {
        println!("You can't drive. Too young!");
    }

    // Multiple conditions with else if:
    let number:i32 = 6;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else {
        println!("Number is neither divisible by 4 nor 3");
    }

    // Using if in a let statement
    let condition:bool = true;
    let num:i32 = if condition {5} else {6};
    println!("Number: {num}");

}

