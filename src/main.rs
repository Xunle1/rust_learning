pub trait Summary {
    fn summarize(&self) -> String {
        String::from("default impl")
    }
    fn describe(&self) {
        println!("decribe Summary")
    }
}

struct Article {
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn main() {
    let a = Article {
        content: String::from("test"),
    };
    a.describe();
    println!("{}", a.summarize());
    notify(&a);
}
