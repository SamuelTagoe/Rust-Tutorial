// Functions
// Entry Points
// Functions and Variables should be written in Snake Case
fn main() {
    greeting();
    tell_height(32);
    human_id("Jhay", 55, 182.0);

    let _x: i32  = {
        let price: i32 = 5;
        let qunatity: i32 = 10;
        price * qunatity    // Should be left without closing it with semi-colunm
    };

    let y:i32 = add(5, 10);
    println!("Value of y = {}", y);
    println!("Value from the function is {}", add(5, 4));

    // Calling the BMI function
    let wieght:f64 = 70.0;
    let height:f64 = 1.82;
    let bmi:f64 = cal_bmi(wieght, height);

    print!("Your BMI is: {:.2}", bmi);
}

// // Global Variables
// //          Any Global var created should be created with Static or Const keyword, not Let
// const _X = {
//     // Code goes here
// };

// Hoisting - Abillity to create a function either below or above the main function and call it anywhere
fn greeting() {
    println!("Hello, Rust🦀!");
}

// Inserting Parameters/input-values
fn tell_height(height:i32) {
    println!("My height is {} cm.", height);
}

// Inserting more than one parameters
fn human_id(name:&str, age:u32, height:f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

// Expressions and Statements
// Expressions: Anything that returns a value
            //  numbers, boolean, functions, conditionals

fn add(a:i32, b:i32) -> i32 {
    a + b
}

// Statements: Anything that doesn't return a value
//             Almost alsways ends with semi-column
// static _x = 10;

// Function to return BMI
// BMI = height(kg)/height(m)^2

fn cal_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}