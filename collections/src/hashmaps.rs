use std::collections::HashMap;

pub fn hasmaps() {
    // Init
    let mut scores = HashMap::new();

    // Add elmts
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get elmts
    let team_name = String::from("Blue");
    // Here, score will have the value that’s associated with the Blue team,
    // and the result will be 10. The get method returns an Option<&V>; if
    // there’s no value for that key in the hash map, get will return None.
    // This program handles the Option by calling copied to get an Option<i32>
    // rather than an Option<&i32>, then unwrap_or to set score to zero if scores
    // doesn’t have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values.

    // If we insert references to values into the hash map, the values won’t be
    // moved into the hash map. The values that the references point to must be
    // valid for at least as long as the hash map is valid.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Updating values
    // Overwriting
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 25);
    // Adding a key and value only if a key isn't present
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Purple")).or_insert(10);

    println!("{scores:?}");

    // Updating value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
