# 包、crate 和 模块

## 概念

- **包（Package）**: Rust 项目的最高层次，一个包通常包含一个 `Cargo.toml` 文件，可以拥有多个二进制 crate 和最多一个库 crate。
- **crate**: Rust 在编译时的最小单位，crate 有两种形式：*binary* 和 *lib*。binary crate 必须有一个 main 函数。lib crate 可以被其他项目引用。
- **模块（mod）和 use**：模块则类似 C++ 中的 *namespace*，用来控制作用域和路径的可见性。

## 包和 crate

使用 `cargo new foo` 可以创建一个包，在创建后会发现路径下有一个 `Cargo.toml` 文件和 `src` 文件夹，`src` 文件夹下有 `main.rs` 文件。

这表示有一个名为 `foo` 的 binary crate，它会按照 `Cargo.toml` 中的规定去构建这个 crate。但是新建的 `Cargo.toml` 中没有定义 `src/main.rc` 文件，原因是 `src/main.rs` 和 `src/lib.rs` 是默认的根 crate。

如果一个包同时含有 `src/main.rs` 和 `src/lib.rs`，这表示它既有 binary crate 也有 lib crate。

## 模块、路径和 use

定义模块

```rust
mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
		fn seat_at_table() {}
	}
	
	mod serving {
		fn take_order() {}
		fn server_order() {}
		fn take_payment() {}
	}
}
```

使用 `pub` 关键字将模块和函数可见性变为公有。可见性规则不仅适用于模块和函数，还适用与结构体、枚举和方法。

如果要使用模块，需要通过路径来引用，比如：

```rust
pub mod front_of_house {
	...
}

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    
    front_of_house::hosting::add_to_waitlist();
}
```

其中第一种是绝对路径，第二种是相对路径。

使用 `use` 将路径引入作用域，类似 C++ 的 using namespace。还可以使用 `as` 重命名。

```rust
// 使用 use 引入路径
use crate::front_of_house::hosting::add_to_waitlist();
// 使用 pub use 重导出名称，使其他作用域也可以使用
pub use crate::front_of_house::hosting
// 使用 as 重命名
use std::fmt::Result;
use std::io::Result as IoResult;
// 嵌套路径
use std::{cmp::Ordering, io};
use std::io::{self, Write};
// 通配符
use std::collections::*;
```
