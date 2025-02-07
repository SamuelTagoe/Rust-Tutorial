// Shadowing
// This is not the same as marking a variable as mut
// shadowing !== (x = 10;)
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x is: {x}")
    }
}

