# 所有权（Ownership）

## 什么是所有权

### 所有权规则

> 1. Rust 中的每一个值都有一个所有者（owner）。
> 2. 值在任一时刻有且只有一个所有者。
> 3. 当所有者（变量）离开作用域，这个值将会被丢弃。

### 内存与分配

```Rust
{
    let s = String::from("hello world"); // 在此处起，s开始有效
}
// s 不再有效
```

当拥有内存的变量离开作用域时，Rust 调用一个特殊函数 **drop** 释放内存。在 C++ 中，在生命周期结束时释放资源的模式又是被称作**资源获取即初始化（*Resource Acquisition Is Initialization(RAII)*）**。

### 移动

```Rust
{
	// error
    let s1 = String::from("hello s1");
    let s2 = s1;

    println!("{}", s1);
}
```

这段代码会报错，因为在 `let s2 = s1` 之后 `s1` 被认为不再有效。这也意味着 Rust 不需要在 `s1` 离开作用域后清理任何东西。类似的，当值被当作参数向函数传递后也可能被移动或者赋值。

### 克隆

```rust
fn main() {
	let x = 5;
	let y = x;

	println!("x = {}, y = {}", x, y);
}
```

这里的 x 依然有效且没有被移动到 y 中，原因是类似整型这样存储在栈上、编译期就确定了大小的类型，可以通过特殊注解 **Copy trait** 使一个变量在赋值给其他变量后还能使用。

## 引用与借用

使用 `&` 来创建一个**引用（reference）**，使用 `*` 来**解引用（dereferencing）**。将创建一个引用的行为叫做**借用（borrowing）**。引用和变量一样，默认都是不可变的。

### 可变引用

使用 `&mut` 可以创建一个**可变引用**，需要该变量也是可变的。

notice：
- 如果已经创建一个对某变量的**可变引用**，就不能再创建对该变量的引用，即不能同时拥有多个单一变量的可变引用。
- 不能再拥有不可变引用的同时拥有可变引用。

```rust
// correct usage
let mut s1 = String::from("hello"); 
let r1 = &s1;
let r2 = &s1;
println!("{} and {}", r1, r2);
let r3 = &mut s3;
println!("{}", r3);
```

## Slice

Slice 是集合中一段连续的元素序列，是一类引用，没有所有权。使用 `[start_index..end_index]` 指定的 range 创建一个 Slice。字符串的字面值就是 slice。
