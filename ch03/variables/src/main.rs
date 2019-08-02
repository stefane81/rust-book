fn main() {

// Rust’s constant naming convention is to use all uppercase with 
// underscores between words, and underscores can be inserted 
// in numeric literals to improve readability.
// const MAX_POINTS: u32 = 100_000;
// println!("The constant is: {}", MAX_POINTS);

let mut x = 5;
x = x+1;
x = x*2;

println!("x is: {}",x);

let spaces = "   ";
let spaces = spaces.len();
println!("spaces is length: {}",spaces);

let guess_ok: u32 = "42".parse().expect("Not a number!");
// let guess_error = "42".parse().expect("Not a number!");

let xx = 2.0; // f64
let yy:f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!("sum:{}\ndifference:{}\nmultiple:{}\ndiv:{}\n%:{}", sum,difference,product, quotient, remainder);

    let t = true;
    let f:bool = false; // explicit type annotation

    println!("t:{}\nf:{}",t,f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c:{}\nz:{}\ncat:{}",c,z,heart_eyed_cat);

    // tuple 
    let tup = (500, 6.4, 1);
    let (xxx, yyy, zzz) = tup;
    println!("The value of y is: {}", yyy);

    let x_tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x_tup.0;
    let six_point_four = x_tup.1;
    let one = x_tup.2;

    println!("c:{}\nz:{}\ncat:{}",c,z,heart_eyed_cat);

    // arrays
    // let a = [1, 2, 3, 4, 5];
    // let aa = ['a';5]; // array of length 5, filled with a
    // let first = a[0];
    // let second = a[1];
    
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
