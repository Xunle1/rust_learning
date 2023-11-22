# 泛型、Trait 和生命周期

## 泛型

当需要对不同类型的参数做处理时，为了减少代码重复，可以使用泛型来抽象替代具体类型。以下介绍几种使用泛型的方法。

### 泛型函数

定义泛型函数的格式：

```rust
fn largest<T>(list: &[T]) -> &T {
	...
}
```

在函数名 `largest` 后添加尖括号并**声明**泛型参数类型 `<T>`，然后在参数列表或返回值列表中使用泛型参数 `(list: &[T]) -> &T`

### 结构体

定义泛型结构体的格式：

```rust
struct Foo<T> {
	x: T,
	y: T,
}
```

需要注意其中 `x` 和 `y` 的类型必须一致，如果想要不一致需要声明两个泛型类型

```rust
struct Bar<T, U> {
	x: T,
	y: U,
}
```

### 枚举

枚举中定义泛型类似结构体，在此之前也接触过使用泛型的枚举 `Option<T>` 和 `Result<T, E>` 等。

```rust
enum Option<T> {
	Some(T),
	None,
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

### 方法

定义泛型方法的格式：

```rust
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}
```

为特定类型实现方法：

```rust
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt();
	}
}
```

### 泛型运行机制

Rust 程序在编译时会找到所有泛型代码被调用的位置，并使用泛型代码针对具体类型生成代码，比如：

```rust
let integer = Some(5);
let float = Some(5.0);
```

这是编译器会生成类似以下代码：

```rust
enum Option_i32 {
	Some(i32),
	None,
}

enum Option_f64 {
	Some(f64),
	None,
}

fn main() {
	let integer = Option_i32::Some(5);
	let float = Option_f64::Some(5.0);
}
```

## trait

`trait` 类似其他语言的接口，用于定义一个或一组方法表示一种或多种行为，不同的类型可以通过实现 `trait` 表示具有某些行为。

### 定义 trait

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}
```

这里定义了一个名为 `Summary` 的 `trait`，它有一个方法 `summarize`，实现了 `Summary` 的类型必须实现 `summarize` 方法。

也有一种情况实现了 `trait` 的类型可以不用实现其中的方法：如果 `trait` 提供了默认实现的话。

```rust
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

fn main() {
    let a = Article {
        content: String::from("test")
    };
    a.describe();
    println!("{}", a.summarize());
}

// describe Summary
// test
```

### trait 作为参数和 trait bound

```rust
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}
```

`notify` 函数使用 `&impl Summary` 指明了参数类型，但 `impl Trait` 其实是一种较长形式的语法糖，称其为 `trait bound`，`impl Trait` 更适用于比较直观简单的参数类型，而 `trait bound` 则相反

```rust
pub fn notify<T: Sumary>(item1: &T, item2: &T) {
	...
}
```

可以通过 `+` 指定多个 `trait`

```rust
pub fn notify<T: Summry + Display>(item: &T) {
	...
}

pub fn notify(item: &(impl Summary + Display)) {
	...
}
```

可以通过 `where` 简化 `trait`

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
	T: Display + Clone,
	U: Clone + Debug,
{
	...
}
```

返回实现了 `trait` 的类型

```rust
fn return_summarizable() -> impl Summary {
	...
}
```

使用 `trait bound` 有条件的实现方法

```rust
impl<T: Display> ToString for T {
	...
}
```

如上示例对实现了 `Display` 的类型实现 `ToString` trait。
