fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Height: {}\tWidth: {}", rect1.height, rect1.width);
    println!("The are of the rectangle is {} square pixels", area(&rect1));
    println!("Height: {}\tWidth: {}", rect1.height, rect1.width);

    println!("Rect: {:?}", rect1);

    // Methods
    println!("The are of the rectangle is {} square pixels", rect1.area());
    rect1.print();

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Call an associated function
    let sq = Rectangle::square(30);
    println!("{:?}", sq);
    println!("Square area: {}", sq.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

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
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Other impl scope
impl Rectangle {
    fn print(&self) {
        println!("Height: {}\tWidth: {}", self.height, self.width);
    }
}
