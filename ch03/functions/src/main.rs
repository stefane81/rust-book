fn main() {
    println!("Hello, world!");

    another_function(5,6);

    let x = 5;

    let y = {
        let x = 3;
        x+1
    };

    println!("x: {}, y: {}",x,y);
    
    println!("expression: {}, function_Return: {}",expression_five(),func_five());
}

fn another_function(x: i32, y:i32) {
    println!("Another func {},{}",x,y);
}

fn expression_five()->i32 {
    5
}

fn func_five()-> i32{
    let x:i32=5;
    return 5;
}