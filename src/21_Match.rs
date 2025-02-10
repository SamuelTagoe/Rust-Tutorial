// Matches acts like if/else

fn main() {

    let some_bool = true;
    match some_bool {
        true => println!("Boolean is true"),
        false => println!("Boolean is false"),
    }

    let some_char = 'a';
    match some_char {
        'a' => println!("Apple"),
        'b' => println!("Banana"),
        _ => println!("Anything else"),
    }

    let numbers = vec![1,2,3,4,5];
    match numbers.is_empty() {
        true => println!("The vector is empty"),
        false => println!("The vector is not empty"),
    }
    
}