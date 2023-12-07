// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     // associate type
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

use crate::List::{Cons, Nil};
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // handle recursive type with Box
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Use Box like a reference
    // let x = 5;
    // let y = &x;
    // let z = Box::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // assert_eq!(5, *z);

    // custom smart pointer
    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, *y);

    // Rc<T> smart pointer
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
