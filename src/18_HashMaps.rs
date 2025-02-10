// Collection Types
// Vectors -- UTF8  -- HashMaps

use std::collections::HashMap;  // To use hash maps, import this
fn main() {

    // HashMaps
    let mut scores = HashMap::new();        // Initialize an instance of a HashMap

    // Inserting into a HashMap
    scores.insert(String::from("Black"), 10);
    scores.insert(String::from("White"), 50);

    let team_name = String::from("Black");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for(key, value) in &scores {
        println!("{key}: {value}");
    }

}
