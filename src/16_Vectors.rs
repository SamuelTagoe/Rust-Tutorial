// Collection Types
// Vectors -- UTF8  -- HashMaps
fn main() {

    // Vectors
    //         They allow you to store more than one value in a single data structure. 
    //         Can only store values of the same type. It is mutable
    
    let _v:Vec<i32> = Vec::new();  // Initialize a vector
    let _the_vec:Vec<i32> = vec![1,2,3];

    // Vectors are dynamic, meaning they can grow or shrink in size.
    let mut _the_numbers_vec:Vec<i32> = Vec::new();

    // Append or Push to a Vector
    _the_numbers_vec.push(1);
    _the_numbers_vec.push(2);
    _the_numbers_vec.push(3);
    _the_numbers_vec.push(4);
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    println!("{:?}", _the_numbers_vec);

    // Read elements from a vector
    let third:&i32 = &_the_numbers_vec[2];  // Getting the element by reference so we don't claim ownership (Direct Indexing)
    println!("The second element is {:?}", third);

    // Getting whatever index needed from a vector
    let var = _the_numbers_vec.get(2);
    match var {
        Some(x) => println!("The third element from a GET METHOD is {}", x),
        None => println!("Index out of bounds"),
    }

    // Iterate over a vector
    for number in &_the_numbers_vec {
        println!("{}", number);
    }

    // Remove elements from a vector
    _the_numbers_vec.remove(1);
    println!("{:?}", _the_numbers_vec);

    // HashMaps
    //         They allow you to store key-value pairs in a single data structure. 
    //         Can store values of different types. It is mutable
    
    let mut _the_hash_map:std::collections::HashMap<&str, i32> = std::collections::HashMap::new();

    // Insert or Update a value in a HashMap
    _the_hash_map.insert("one", 1);
    _the_hash_map.insert("two", 2);
    _the_hash_map.insert("three", 3);

}
