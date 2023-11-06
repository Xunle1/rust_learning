// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?} ==> call", self);
//     }

//     fn display() {
//         println!("Message::display");
//     }
// }

fn main() {
    // let m = Message::Write(String::from("hello enum"));
    // println!("{:?}", m);
    // m.call();
    // Message::display();

    // ========= Option =========
	// let num = Some(5);
	// let ch = Some('e');
	// let absent: Option<String> = None;
    // assert_eq!(num.is_some(), true);
	// assert_eq!(ch.is_none(), true);
	// assert_eq!(absent.is_none(), true);
    
    // ========= match ========
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i+1),
    //     }
    // }
    // let five = Some(5);
	// let six = plus_one(five);
	// let none = plus_one(None);
	// println!("{}", six.unwrap());
	// println!("{}", none.unwrap());

    let config_max = Some(10);
    // let none: Option<String> = None;
	if let Some(max) = config_max {
		println!("The maximum configured num is {}", max);
	} else {
		println!("not configured maximum num");
	}
}
