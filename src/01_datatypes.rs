// Primitive data types
// int, float, bool, char

// Integer ===============================================
// Rust has signed (+ and -) and unsigned int (only+) types of different sizes
// Signed :
//          i8, i16, i32, i64, i128  == (-)
// Un-signed :
//          u8, u16, u32, u64, i128 == (+)

// Floats ===============================================
//[Floating Point Types] : f32, f64

fn main() {
    // Ints
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Floats
    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean Values: true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet = {}", letter);
}

