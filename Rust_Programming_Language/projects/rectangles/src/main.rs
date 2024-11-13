// Original code
//fn main() {
//    let width1 = 30;
//    let height1 = 50;
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(width1, height1)
//    );
//}
//
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

// With tuples
//fn main() {
//    let rect1 = (30, 50);
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(rect1)
//    );
//}
//
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}

// With structs
// derive(Debug) is needed to use {:?} and {:#?} to print out structs without individually calling
// out fields
//#[derive(Debug)]
//struct Rectangle {
//    width: u32,
//    height: u32,
//}
//
//impl Rectangle {
//    fn area_method(&self) -> u32 {
//        self.width * self.height
//    }
//}
//
//impl Rectangle {
//    fn width(&self) -> bool {
//        self.width > 0
//    }
//}
//
//fn main() {
//    //let rect1 = Rectangle {
//    //    width: 30,
//    //    height: 50,
//    //};
//
//    let scale = 2;
//    let rect1 = Rectangle {
//        width: dbg!(30 * scale),
//        height: 50,
//    };
//
//    // Another way to print out debug statements
//    // This is different with regards to ownership to println and prints to stderr instead of
//    // stdout
//    dbg!(&rect1);
//
//    println!(
//        "The area of the rectangle from function is {}  and method is {} square pixels.",
//        area(&rect1), rect1.area_method()
//        );
//
//    println!("rect1 is {rect1:?}");
//    println!("rect1 is {rect1:#?}");
//
//    if rect1.width() {
//        println!("The rectangle has a nonzero width: it is {}", rect1.width);
//    }
//}
//
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

// mini project
// This derive will allow set_to_max compile
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// One or multple impl does not matter, but why use multiple?
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // want it to be an immutable borrow since it should be read only and want main to keep
    // ownership of other instances of Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions, don't have Self as parameter
    // Often used as a constructor: new is not special in Rust as a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        //let max = self.max(other);
        *self = self.max(other);
    }
}

fn main() {
    //let rect1 = Rectangle {
    //    width: 30,
    //    height: 50,
    //};
    //let rect2 = Rectangle {
    //    width: 10,
    //    height: 40,
    //};
    //let rect3 = Rectangle {
    //    width: 60,
    //    height: 45,
    //};

    //println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    //println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
    //
    //let sq = Rectangle::square(3);

    //println!("Dimensions of sq are {}, {}", sq.width, sq.height);

    //// Why methods are called syntactic sugar, the following are equivalent calls
    //let mut r = Rectangle {
    //    width: 1,
    //    height: 2
    //};
    //let area1 = r.area();
    //let area2 = Rectangle::area(&r);
    //assert_eq!(area1, area2);

    //r.set_width(2);
    //Rectangle::set_width(&mut r, 2);

    //// Rust will insert ref/deref to make types match up in self parameter
    //let r2 = &mut Box::new(Rectangle {
    //    width: 1,
    //    height: 2
    //});
    //let area3 = r2.area();
    //let area4 = Rectangle::area(&**r2);
    //assert_eq!(area3, area4);
    let rect = Rectangle {
        width: 0,
        height: 0
    };

    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1
    };

    let max_rect = rect.max(other_rect);

    println!("{}", max_rect.area());
}
