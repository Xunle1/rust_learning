# 结构体

## 结构体定义及使用

**定义**

```rust
struct User {
	name: String,
	email: String,
	active: bool,
	sign_in_count: u64,
}
```

**实例化、使用、更新**

```rust
fn main() {
    let mut user1 = User {
        name: String::from("xunle"),
        email: String::from("xunle1121@gmail.com"),
        active: true,
        sign_in_count: 1,

    };
    println!(
        "user: {}, {}, {}, {}",
        user1.name, user1.email, user1.active, user1.sign_in_count
    );

    user1 = build_user(String::from("test build"), String::from("test@foo.com"));
    println!(
        "user: {}, {}, {}, {}",
        user1.name, user1.email, user1.active, user1.sign_in_count
    );

    let user2 = User {
        name: String::from("dadan"),
        email: String::from("dadan@foo.com"),
        ..user1
    };

    println!(
        "user: {}, {}, {}, {}",
        user2.name, user2.email, user2.active, user2.sign_in_count
    );
}

fn build_user(name: String, email: String) {
	User {
		name, 
		email,
		active: false,
		sign_in_count: 0,
	}
}

// output
// user: xunle, xunle1121@gmail.com, true, 1
// user: test build, test@foo.com, false, 0
// user: dadan, dadan@foo.com, false, 0
```

**通过 trait 增加结构体功能**

```rust
#[derive(Debug)]
struct User {
	name: String,
	email: String,
	active: bool,
	sign_in_count: u64,
}

fn main() {
    // trait
    let user3 = User {
        name: String::from("bar"),
        email: String::from("bar@foo.com"),
        active: false,
        sign_in_count: 2,
    };
    println!("{:#?}", &user3);
}

// output
// User {
//     name: "bar",
//     email: "bar@foo.com",
//     active: false,
//     sign_in_count: 2,
// }
```

## 方法

方法的定义需要使用关键字 `impl`，并且方法和函数有着相似的样子，区分的方法是方法的第一个参数总是 `&self` 或 `&mut self`，`self` 表示调用这个方法的实例引用

**关联函数**

在 `impl` 中定义的函数叫做关联函数。使用 `::` 解析符调用关联函数。

```rust
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
}

fn main() {
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

}
```
