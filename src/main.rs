fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);

    // let s1 = String::from("hello s1");
    // let s2 = s1.clone();
    // println!("s1: {}, s2: {}", s1, s2);

    // func_ownership(s);
    // println!("after function: {}", s);

    // reference and borrowing
    // let mut s1 = String::from("hello");
    // let len = calcuate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let r1 = &s1;
    // let r2 = &s1;
    // println!("{} and {}", r1, r2);
    // let r3 = &mut s1;
    // println!("{}", r3);

    // slice
    // let s = String::from("hello rust");
    // let w = first_word_index(&s);
    // println!("The first word of '{}' is '{}'", s, w);

    let a = [1,2,3,4,5];
    let b = a;

    let mut index = 0;
    while index < 5 {
        println!("a[{}]: {}, b[{}]: {}", index, a[index], index, b[index]);
        index += 1;
    }

}

// fn func_ownership(s: String) {
//     println!("{}", s);
// }

// fn calcuate_length(s: &String) -> usize {
//     s.len()
// }

// fn first_word_index(s: &String) -> &str {
//     let bytes = s.as_bytes();
    
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }