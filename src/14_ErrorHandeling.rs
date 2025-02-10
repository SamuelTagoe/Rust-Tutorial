// Error Handling Techniques
fn main() {

    // First Technique ********************************
    // OPTION T Tecnique is an Enum 
    // It's a tool for error handling, this is used when a value might not be present
    // Going to avoid the pitfalls of Null Reference Error

    // enum Option<T>{     // Defining the generic Option type
    //     Some(T), // Represents a value
    //     None,   // Represents no value

    //     // The option is going to check something and if the something is positive, 
    //     // it's will return Some with that type, if not the return None
    // }

    // Second Technique ********************************
    // RESULT T Technique, also an Enum
    // It's used for operations that can succeed with the Ok(T)
    // Incase of failure, returns Err(E), which means Error and the equivalent error typr (e)

    // enum Result<T, E> {     // Defining the generic Result type
    //     Ok(T), // Represents a successful operation with a value
    //     Err(E), // Represents a failed operation with an error
    // }

    // Example of Option Approach
    // Evoke Divide Function
    let result = divide(10.0, 0.0);
    match result {
        Some(x) => println!("{}", x),
        None => println!("Error: Division by zero"),
    }

    // Example of Result Approach
    match div(10.0, 2.0) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }


    

}

// Function to implement Option
fn divide(num:f64, denum:f64) -> Option<f64> {
    if denum == 0.0 {
        None
    } else {
        Some(num / denum)
    }
}

// Function to implement Result
fn div(num: f64, denum: f64) -> Result<f64, String> {
    if denum == 0.0 {
        Err("Can't dividie by 0".to_string())
    } else {
        Ok(num / denum)
    }
}