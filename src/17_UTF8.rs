// Collection Types
// Vectors -- UTF8  -- HashMaps
fn main() {

    // UTF8 is all about Strings
    // Basically storing UTF encoded text into Strings

    // Different ways in Declaring String
    let _s1 = "whatever".to_string();
    let _s2 = String::from("whatever");
    let mut _s3 = String::from("foo");
    let _s = _s1 + &_s2; // This takes comnplete owenership of _s1, 
                                // making the original _s1 non-exsisting, and borrowing _s2 with the use of reference
    

    // Pushing a string slice
    _s3.push_str("bar");

    // Pusinh a Char
    _s3.push('!');

}
