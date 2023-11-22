# 数据类型

Rust 有两种数据类型子集：标量（scalar）和复合（compound）

## 标量
### 整型

| 长度    | 有符号 | 无符号 |
| ------- | ------ | ------ |
| 8-bit   | i8     | u8     |
| 16-bit  | i16    | u16    |
| 32-bit  | i32    | u32    |
| 64-bit  | i64    | u64    |
| 128-bit | i128   | u128   |
| arch    | isize  | usize  |

### 浮点型

Rust 有两种浮点数类型 `f32` 和 `f64`，默认是 `f64`

### 布尔型

`bool` 类型两个可能的值：`true`、`false`

### 字符类型

`char` 类型大小为四个字节

## 复合类型

### 元组类型

元组（Tuple）是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定，声明后长度不会改变

**组合**

```rust
let tup1 = (1, 2, 3);
let tup2: (i32, f64, u8) = (500, 6.4, 1);
```

**解构**

```rust
let (x, y, z) = tup2;
```

使用 `.` 来访问元组元素，下标从 0 开始

```rust
let x = tup2.0;
let y = tup2.1;
let z = tup2.2;
println!("1: {tup2.0}, 2: {tup2.1}, 3: {tup2.2}");
// 1: 500, 2: 6.4, 3: 1
```
### 数组类型

数组（Array）包含一组相同元素，同样长度固定不可改变

```rust
let a1 = [1,2,3]; // 1, 2, 3
let a2 = [3; 5]; // 3, 3, 3, 3, 3
let a3 = [f64: 3] = [1.2, 3.6, 9.7];
```

通常使用 vector。

# 函数

Rust 函数的格式为

```rust
fn func_name (arg1: i32, arg2: char) -> i32 {
    ...
}
```

语句（Statements）没有返回值，表达式（Expressions）有返回值。

函数的返回值等同于函数体最后一个表达式的值，最后一个表达式不能加分号。

# 流程控制

## if 语句

```rust
if bool_value {
    ...
} else {
    ...
}
```

因为 `if` 是一个表达式，所以可以在 `let` 的右侧使用：

```rust
let condition = true;
let num = if condition { 5 } else { 6 };
println!("The value of number is: {num}");
```

分支中的返回值应该是同一种类型

## 循环语句

Rust 提供三种循环语句，`loop`、`while`、`for`

```rust
loop {
	...
}
```

```rust
while condition {
	...
}
```

```rust
for elem in arr {
	...
}
```
