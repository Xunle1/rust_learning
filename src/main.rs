fn main() {
    // assign a mutable variable
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // constant
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("three hours equals {THREE_HOURS_IN_SECONDS} seconds")

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of x is {x}, y is {y}, z is {z}");

    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
    // println!("{five_hundred}, {six_point_four}, {one}")

    // Array
    // let a = [3; 5]; // 声明数组大小为5，初始值为3
    // let a1 = a[1];
    // println!("{a1}");

    // Function
    // let (x, y) = plus_one(1, 2);
    // println!("x is {x}, y is {y}");

    
    // Control Flow
    control_flow()
}

// fn plus_one(x: i32, y: i32) -> (i32, i32) {
//     (x + 1, y + 1)
// }

fn control_flow() {
    // if example
    // let flag = true;
    // let num = if flag { 5 } else { 6 };
    // println!("The value of num is {num}")

    // loop example
    // let mut counter = 0;
    // let y = loop {
    //     counter += 1;
        
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The value of y is {y}");

    // let mut counter = 0;
    // 'counting_up: loop {
    //     println!("count: {counter}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining: {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }

    //         if counter == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }
    //     counter += 1;
    // }
    // println!("End count: {counter}")

}