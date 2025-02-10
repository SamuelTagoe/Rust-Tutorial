// Matches acts like if/else

fn main() {

    let numbers = vec![1,2,3,4,5];
    match numbers.is_empty() {
        true => println!("The vector is empty"),
        false => println!("The vector is not empty"),
    }
    
}