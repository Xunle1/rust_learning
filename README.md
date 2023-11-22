# 错误处理

Rust 中将错误分为两大类：**可恢复**和**不可恢复**错误。通常，使用 `panic!` 宏来处理不可恢复错误，使用 `Result<T, E>` 类型来处理可恢复错误。

## Panic

在 Rust 程序中，有两种导致 panic 的情况，一是手动调用 `panic!` 宏，二是进行师代码 panic 的操作（比如数组越界访问）。

在发生 panic 后，可以通过设置 `RUST_BACKTRACE` 为 0 以外的值来打印 panic 时的调用栈信息。

## Result

使用 `Result<T, E>` 类型来处理不可恢复错误，`Result` 也是一个枚举类型，和 `Option` 一样在 preclude 中，所以不用特地引入。它使用 `Ok(T)` 和 `Err(E)` 来分别返回

- 成功时的返回类型T的值
- 失败时的错误类型E的值

### 使用 match 处理结果、匹配错误

使用 match 处理 Result。

```rust
let result = File::open("hello.txt");
let file match file {
	Ok(f) => f,
	Err(err) => panic!("failed to open file: {:?}", err),
};
```

使用 match 处理错误类型

```rust
let result = File::open("hello.txt");
let file = match file {
	Ok(f) => f,
	Err(err) => match err.kind() {
		ErrorKind::NotFound => match File::create("hello.txt") {
			Ok(fc) => fc,
			Err(e) => panic!("cannot create file: {:?}", e),
		},
		other_error => {
			panic!("failed to open file: {:?}", other_error);
		}
	},
};
```

### 使用 unwrap 和 expect 处理失败时 panic

Result 提供了辅助方法来处理各种情况。上面两个代码片段都在 Result 返回错误时进行了 panic 操作，对于这种情况可以使用 `unwrap` 和 `expect` 来处理。

```rust
// use unwrap
let file = File::open("hello.txt").unwrap();
// use expect
let file = File::open("hello.txt").expect("failed to open file");
```

expect 可以自定义 panic 时的提示信息。

### 使用 ? 运算符传播错误

使用 `?` 运算符，如果 Result 的值不是错误，则表达式会返回 `Ok` 中的值，如果是 `Err` 的话，则会将 `Err` 作为整个函数的返回值返回给调用者。

```rust
fn read_username_from_file() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

`?` 只能用在返回值为 `Result<T, E>`、`Option<T>` 或实现了 `FromResidual` 类型的函数中。
