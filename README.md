# 测试

在 Rust 中可以简单将测试分为单元测试和集成测试两种类型，这两种类型的测试在编写过程中有一些差异。

## 单元测试

使用一段代码来说明单元测试的组成部分，首先使用 `cargo new my_tests --lib` 新建项目，在 `src/lib.rs` 下：

```rust
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
```

在这段代码中，`#[cfg(test)]` 注解表示这段代码只在测试时才会编译运行。`#[test]` 表示这是一个测试函数。`assert_eq!(result, 4)` 则是一个断言，如果符合断言内容则测试通过，否则测试失败。

### 使用 `assert!` 断言

简单的使用方法是直接将需要判断的结果作为参数判断，比如：

```rust
assert!(result == 4);
assert!(isValid(99));
```

其中 `isValid` 返回一个 `bool` 类型。`assert!` 还可以定义失败信息：

```rust
let v = 99;
assert!(isValid(v), "{} is not valid!", v);
```

使用 `#[should_panic]` 检查 panic：

```rust
#[test]
#[should_panic]
fn it_works() {
	panic("something happens.");
}
```

如果有 panic 发生，测试通过，否则测试失败。也可以指定特殊信息的 panic：

```rust
#[test]
#[should_panic(expected = "sub sentence")]
fn it_works() {
	panic("something happens. sub sentence...");
}
```

panic 错误信息如果包含 `expected` 中的值，测试通过。

## 集成测试

在 Rust 中，可以在项目根目录中创建 `tests` 目录，和 `src` 目录同级，并在 `tests` 目录下创建测试代码来测试库的公用 API。比如项目结构是：

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

在 `integration_test.rs` 中编写测试代码：

```rust
use my_tests;

#[test]
fn test_add_two() {
	let ret = my_tests::add_two(1);
	assert_eq(ret, 3);
}
```

`lib.rs` 中有函数定义：

```rust
pub fn add_two(v: i32) -> i32 {
	v + 2;
}
```

使用 `cargo test` 进行测试会展示测试结果。

如果在集成测试中需要使用到子模块，因为在 `tests` 目录下的每个文件都会被认为是一个 `crate`，所以不能像 `src` 目录那样引入同级的模块来使用模块的功能。即项目结构不能是：

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common.rs
    └── integration_test.rs
```

需要更改为：

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │    └── mod.rs
    └── integration_test.rs
```

才能在 `integration_test.rs` 中调用：

```rust
use my_tests;

mod common;

#[test]
fn test_add_two() {
	common::setup();
	let v = tests::add_two(1);
	assert_eq(v, 3);
}
```

## 控制测试运行

指定测试线程数量：

```shell
cargo test -- --test-thread=1
```

显示函数输出：

```shell
cargo test -- --show-output
```

指定名称运行部分测试：

```shell
cargo test $TESTS_NAME
```

这条命令会运行名称包含 `$TESTS_NAME` 在内的模块和函数。

忽略某些测试：

```rust
#[test]
#[ignore]
fn test () {
	...
}
```

使用 `#[ignore]` 注解来排除测试，只运行被忽略的测试和包括被忽略的测试一起执行使用一下命令：

```shell
cargo test -- --ignored
cargo test -- --include-ignored
```
