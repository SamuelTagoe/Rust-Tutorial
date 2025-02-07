// Constants
// They can never be mutable
// constants should be declared with upper case

fn main() {
    let mut x = 10;
    const Y:i32 = 19;

    print!("Value of X = {}", x);
    print!("Value of Y = {}", Y);
    print!("Value of Pi = {}", PI);
    print!("Value of 3 hours in seconds = {}", THREE_HOURS_IN_SECONDS);
}


// A const can be declared outside the Global Scope
const PI:f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;