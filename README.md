# 枚举

## 枚举的定义和使用

```rust
#[derive(Debug)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("{:?} ==> call");
	}

	fn display() {
		println!("Message::display");
	}
}

fn main() {
	let m = Message::Write(String::from("hello enum"));
	println!("{:?}", m);
	m.call();
	Message.display();
}

// output:
// Write("hello enum")
// Write("hello enum") ==> call
// Message::display 
```

## Option 枚举

Rust 并没有空值，不过 Rust 使用 `Option<T>` 枚举来标识存在和不存在的概念。

```rust
enum Option<T> {
	None,
	Some(T),
}
```

`Option<T>` 已经被包含在 prelude 之中，不需要显示引用。

```rust
fn main() {
    let num = Some(5);
    let ch = Some('e');
    let absent: Option<String> = None;

    assert_eq!(num.is_some(), true);
    assert_eq!(ch.is_none(), true);
    assert_eq!(absent.is_none(), true);
}

// output
// thread 'main' panicked at src\main.rs:28:5:
// assertion `left == right` failed
//   left: false
//  right: true
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\hello_rust.exe` (exit code: 101)
```

## match 控制流

`match` 关键字允许将一个值和一系列的模式相比较，可类比其他语言中的 `switch`。

匹配枚举

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{}", six.unwrap());
    println!("{}", none.unwrap());
}

// output
// 6
// thread 'main' panicked at src\main.rs:44:25:
// called `Option::unwrap()` on a `None` value
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\hello_rust.exe` (exit code: 101)
```

Rust 的匹配时有穷尽的，如果没有将模式的所有可能值匹配完全，会抛出一个错误。可以使用通配符 `_` 或 `other` 来表示其他值。

## if let 控制流

`if let` 可以认为时 `match` 的一个语法糖，`if let` 可以匹配一个表达式和模式并忽略其他模式或者使用 `else` 对其他模式做处理。

```rust
fn main() {
	let config_max = Some(10);
	if let Some(max) = config_max {
		println!("The maximum configured num is {}", max);
	} else {
		println!("not configured maximum num");
	}
}
```
