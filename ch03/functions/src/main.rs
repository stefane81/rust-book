fn main() {
    println!("Hello, world!");

    another_function(5,6);

    expressives();
    loop_loop();
    while_loop();
    for_loop();

}

fn another_function(x: i32, y:i32) {
    println!("Another func {},{}",x,y);
}

fn expressives() {
    let x = 5;

    let y = {
        let x = 3;
        x+1
    };

    println!("x: {}, y: {}",x,y);
    
    println!("expression: {}, function_Return: {}",expression_five(),func_five());
}

fn expression_five()->i32 {
    5
}

fn func_five()-> i32{
    let x:i32=5;
    return 5;
}

fn loop_loop(){
    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!("{}!",number);

        number -= 1;
    }

    println!("LIFTOFF");
}

fn for_loop(){
    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("{}",element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}