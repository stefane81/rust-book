#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30,50);
    let rect2 = Rectangle {width:30, height: 50};

    println!("The area of the retangle is {} square pixels.",
        area(width1,height1));

    println!("The area of the retangle is {} square pixels.",
        area_tuple(rect1));

    println!("The area of the retangle is {} square pixels.",
    area_struct(&rect2));

    println!("rect1 is {:#?}", rect1);
}

fn area(width:u32, height:u32) -> u32 {
    width*height
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}