fn main() {
    println!("Hello, world!");
    let mut _v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    println!("{:?}", _v);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut vmut = vec![1, 2, 3];

    for i in &mut vmut {
        *i *= 50;
        println!("{}", i);
    }

    // -----

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
