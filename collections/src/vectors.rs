pub fn vectors() {
    // Init new vector
    let _: Vec<i32> = Vec::new();
    // Init using macro
    let mut v = vec![1, 2, 3];

    {
        let v = vec![10, 11, 12];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Add vals
    v.push(4);
    v.push(5);
    v.push(6);

    // Reading elements of a vector
    // Using indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    // Using get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Using loops
    for i in &v {
        println!("{i}");
    }

    let mut vec = vec![100, 32, 57];

    for i in &mut vec {
        *i += 50;
    }

    // Can store "different types" in a vector by grouping them
    // within an enum!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Hello, world!")),
        SpreadsheetCell::Float(2.2),
    ];
}
