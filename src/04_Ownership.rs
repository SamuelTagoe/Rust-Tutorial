// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Management Control Issues
// Garbage Collector solved this issue, but created a new issue
// [stopping/resuming the program]

// OWNERSHIP intoduced by RUST to solve memory safety issues and high performace at the same time.
// What is OWNERSHIP?
// Every value has a single owner [every variable has one value, and it is it's sole owner].
// Ownershipe Rules
/*
    1 - Each value in Rust has a variable that's its owner
    2 - There can be only one owner at a time
    3 - When the ownber goes out of scope, the value will be dropped
*/


fn main() {

    // // Rule 1
    // let s1 = String::from("RUST");
    // let len = calc_len(&s1);
    // println!("Length of '{}' is {}", s1, len);

    // // Rule 2
    // let s1 = String::from("RUST");
    // let s2 = s1;
    // println!("{}", s1)

    // // Rule 3
    // let s1 = String::from("RUST");
    // let len = calc_len(&s1);
    // println!("{}", len);
}

// fn calc_len(s:&String) -> usize {
//     s.len()
// }
