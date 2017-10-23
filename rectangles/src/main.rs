#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect: Rectangle = Rectangle {
        length: 50,
        width: 30,
    };

    println!("The rectangle is {:#?}", rect);
    println!("The area is {}", area(&rect));
}

fn area(r: &Rectangle) -> u32 {
    r.length * r.width
}
