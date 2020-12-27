#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width> other.width && self.height>other.height

    }

    fn squre(size: u32) -> Rectangle {
        Rectangle { width:size, height:size }
    }
}

fn main() {

    let rect1 = Rectangle {width: 30 , height: 20 };
    let rect2 = Rectangle {width: 40 , height:40 };
    let rect3 = Rectangle::squre(10);
    println!("rect3 is {:#?}", rect3);
    println!("rect1 is {:#?}",rect1);
    println!("rect1 area is { }",rect1.area());

    println!("rect2 can hold rect1 ? { }", rect2.can_hold(&rect1));
   
}
