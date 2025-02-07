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

    let animal_slices:&[&str] = &["Monkey", "Cat", "Goat"];
    println!("Animal Slice: {:?}", animal_slices);

    // This case I want a string, not a string slice
    let book_slice:&[&String] = &[&"Programming".to_string(), &"Tech".to_string(), &"Story".to_string()];
    println!("Book Slice: {:?}", book_slice);

    // Strings VS String Slices (&str)
    /*
    Strings are expandable, can be increased and decreased, 
            basically mutubale. They are owned string types, meaning they arent borrowed. Stroed on the Heap

    String Slices are opposite of Strings, are immutable
                  Used to refrence string objects without needing to copy or own the data
                  Very good for memory efficiency, so can work with data without taking ownershp of it 

    Stack is quicker Heap is slower
    */

    // String
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}",  stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value {}", slice);

}

