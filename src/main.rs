// #[derive(Debug)]
// struct User {
//     name: String,
//     email: String,
//     active: bool,
//     sign_in_count: u64,
// }

#[derive(Debug, Copy, Clone)]
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

fn main() {
    // let mut user1 = User {
    //     name: String::from("xunle"),
    //     email: String::from("xunle1121@gmail.com"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // println!(
    //     "user: {}, {}, {}, {}",
    //     user1.name, user1.email, user1.active, user1.sign_in_count
    // );
    // user1 = build_user(String::from("test build"), String::from("test@foo.com"));

    // println!(
    //     "user: {}, {}, {}, {}",
    //     user1.name, user1.email, user1.active, user1.sign_in_count
    // );
    // let user2 = User {
    //     name: String::from("dadan"),
    //     email: String::from("dadan@foo.com"),
    //     ..user1
    // };
    // println!(
    //     "user: {}, {}, {}, {}",
    //     user2.name, user2.email, user2.active, user2.sign_in_count
    // );

    // trait
    // let user3 = User {
    //     name: String::from("bar"),
    //     email: String::from("bar@foo.com"),
    //     active: false,
    //     sign_in_count: 2,
    // };
    // println!("{:#?}", &user3);

    // method
    let r1 = Rectangle {
        width: 2,
        height: 4,
    };
    let r2 = Rectangle {
        width: 1,
        height: 3,
    };
    println!("{:?} can hold {:?} ? {}", r1, r2, r1.can_hold(&r2));

    // associated function
    let square = Rectangle::square(3);
    println!("square {:?}'s area is {} ", &square, square.area());
}

// fn build_user(name: String, email: String) -> User {
//     User {
//         name,
//         email,
//         active: false,
//         sign_in_count: 0,
//     }
// }
