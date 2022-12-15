# 常用编程概念

## 变量

### 可变性
变量默认是不可变（immutable）的，使用 `mut` 可以是变量可变

### 常量
使用 `const` 关键字声明常量，需指定类型和值，可以在任意作用域声明

## 数据类型
Rust 有两种数据类型子集：标量（scalar）和复合（compound）

### 整型
| 长度    | 有符号 | 无符号 |
|---------|--------|--------|
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

### 元组类型
元组（Turple）是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定，声明后长度不会改变

### 数组类型
数组（Array）包含一组相同元素，同样长度固定不可改变

## 函数
Rust 函数的格式为
```rust
fn func_name (arg1: i32, arg2: char) -> i32 {
    ...
}
```
Rust 中，语句（Statements）没有返回值，表达式（Expressions）有返回值。

## 流程控制

### if 语句
```rust
if bool_value {
    ...
} else {
    ...
}
```
因为 `if` 是一个表达式，所以可以在 `let` 的右侧使用：
```rust
let num = if condition { 5 } else { 6 };
```
分支中的返回值应该是同一种类型

### 循环语句
Rust 提供三种循环语句，`loop`、`while`、`for`

