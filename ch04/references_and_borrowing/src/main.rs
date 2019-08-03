fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);

    // change(&s1); //`some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", mutation");
}