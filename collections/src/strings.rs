pub fn strings() {
    let mut s = String::from("foo");
    let s2 = "bar";
    // push_str does not take ownership of s2
    s.push_str(s2);
    println!("s2 is {s2}");

    // Appending a single char
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // Concatenating with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Can get messy with many strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    // Use format! macro instead
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    // The best way to operate on pieces
    // of strings is to be explicit about
    // whether you want characters or bytes.
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
