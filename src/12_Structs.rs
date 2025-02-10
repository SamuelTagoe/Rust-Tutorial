// Stucts
//          Used to name and package together related values to similar to tuples

fn main() {

    // tuple
    let rec: (i32, i32) = (200,500);

    // Struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: String,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        password: String,
        sign_in_count: u64,
    }

    // The entire struct instance must be mutable
    let mut user1 = User {
        active:  true,
        username: String::from("Jhay"),
        email: String::from("jay@example.com"),
        password: String::from("password123"),
        sign_in_count: 10,
    };

    user1.email = String::from("jhay@example");
    println!("User1: {}", user1.email);

    // Return a struct from a Function
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            email,
            username,
            password: String::from("password123"),
            sign_in_count: 0,
        }
    }

    // Create instances from other instances
    let mut user2 = User {
        email: String::from("user2@example.com"), // This gives a unique email for user2
        ..user1     // To keep all of other values the same as user1
    };

    // Tuple Structs
    // They are like other structs but the main difference is that they don't have any name fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Instances of the Tuple Struct
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // println!("Black: {:?}", black);
    // println!("Origin: {:?}", origin);

    // Unit-Like Structs
    // Are very specific, they have no fields, 
    // and are used when you need a type to implement a trait but don't need to store data
    // They are used when you don't want to have any data inside them
    struct AlwaysEqual;

    let subject = AlwaysEqual;

}

