// Looping Mechanism
// Executing a block of code repeatedly until you explicitly tell it to stop
// Rust offers only 3 types of Loops
/*
    - Loops
    - While Loops
    - For Loops
*/

fn main() {

    // Loops ****************************************************************
    loop {
        println!("Hello World");
    }

    let mut counter:i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter - 100;
        }
    };
    println!("The result is: {result}");

    // While Loops ****************************************************************
    let mut number:i32 = 3;
    while number != 0 {
        println!("Number: {}", number);
        number -= 1;
        // break;
    }
    println!("HEY!!!");

    // For Loops ***************************************************************
    let a = [1,2,3,4,5,6];
    let b = ["a", "b", "c"];

    for element in a {
        println!("{element}");
    }

    for letter in b {
        println!("{letter}");
    }

}

