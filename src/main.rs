// Compound Data Types
// arrays, tuples, slices and strings (slice string)

fn main() {
    // Arrays
    let numbers:[i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    let fruits:[&str; 3] = ["Apple", "Banana", "Orange"];
    println!("First Fruit: {:?}", fruits[0]);
    println!("First Fruit: {:?}", fruits[1]);
    println!("First Fruit: {:?}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Tuples = {:?}", human);

    // Mix Tuple
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix TupleL {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices:&[i32; 5] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);
}

