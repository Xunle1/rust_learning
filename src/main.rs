fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello s1");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    func_ownership(s);

}

fn func_ownership(s: String) {
    println!("{}", s);
}
