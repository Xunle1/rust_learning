// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
//
// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
//
// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }
//
//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
//
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

use std::thread;

fn main() {
    // Closure Example.
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    // };
    //
    // let user_pref1 = Some(ShirtColor::Blue);
    // let color1 = store.giveaway(user_pref1);
    // println!("The user preference {:?} gets {:?}", user_pref1, color1);
    //
    // let user_pref2 = None;
    // let color2 = store.giveaway(user_pref2);
    // println!("The user preference {:?} gets {:?}", user_pref2, color2);

    // Declare and call closure.
    // let list = vec![1, 2, 3];
    // let only_borrows = || println!("From closure: {:?}", list);
    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // Mutable Borrow
    // let mut list = vec![1, 2, 3];
    // println!("Before calling closure: {:?}", list);
    // let mut borrows_mutably = || list.push(7);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    // Ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
}

#[test]
fn iterator_sum() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}