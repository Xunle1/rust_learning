# 智能指针

## 使用 `Box<T>` 指向堆上的数据

最简单的智能指针是 box，其类型是 `Box<T>`。 box 允许将一个值放在堆上而不是栈上。 留在栈上的则是指向堆数据的指针。

### box 的使用

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
// output
// 5
```

### Box 创建递归类型

因为 Rust 需要在编译时知道类型占用多少空间，所以当构建一个递归类型时，使用相同类型的值会导致报错。

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

这里创建了 `List` 枚举，它的成员 `Cons` 又包含了 `List` 类型

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

编译这段代码会得到错误： `error[E0072]: recursive type 'List' has infinite size`

### 使用 `Box<T>` 给递归类型一个已知大小

将错误的递归类型代码改造一下

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

此处 `Cons` 成员将会需要一个 `i32` 的大小加上存储 box 指针数据的空间，编译器可以计算出存储一个 `List` 值需要的大小。

## Deref trait

实现 `Deref trait` 允许我们重载 **解引用运算符** `*`， 

### 像引用一样使用 `Box<T>`

```rust
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}
```

### 自定义智能指针

从根本上说，`Box<T>` 被定义为包含一个元素的元组结构体。接下来以相同方式定义 `MyBox<T>` 类型。

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new<T>(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
```

使用 `MyBox<T>`

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

得到的编译错误：`error[E0614]: type 'MyBox<{integer}>' cannot be dereferenced`

原因是 `MyBox<T>` 类型没有实现 `Deref trait`

### 实现 Deref trait

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

当在输入 `*y` 时，Rust 事实上在底层运行了如下代码：

```rust
*(y.deref())
```

## `Rc<T>` 引用计数智能指针

当一个值可能会有多个拥有者的时候，需要使用类型 `Rc<T>`，Rc 只能接收不可变引用，通过不可变引用使多个使用
者共享只读数据。

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    // Rc<T> smart pointer
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

## `RefCell<T>` 和内部可变性模式

**内部可变性**允许你即使在有不可变引用的地方也可以修改数据。可以使用 `RefCell<T>` 在运行时记录借用，
但如果在运行时违反了借用规则会抛出 panic。

虽然 `push_to_immut` 参数列表为 `&ImmutOuter` 不可变引用，但通过 RefCell 的 `borrow_mut()` 
获取了可变引用。

```rust
struct ImmutOuter {
    inner: RefCell<Vec<String>>,
}

impl ImmutOuter {
    fn new() -> ImmutOuter {
        ImmutOuter {
            inner: RefCell::new(vec![])
        }
    }
}

fn push_to_immut(immut: &ImmutOuter) {
    let mut ref_mut = immut.inner.borrow_mut();
    ref_mut.push(String::from("2"));
}

fn main() {
    let immut: ImmutOuter = ImmutOuter::new();
    println!("before push: {:?}", immut.inner.borrow());
    push_to_immut(&immut);
    println!("after push: {:?}", immut.inner.borrow());
}
```

运行上面代码片段得到：

```
before push: []
after push: ["2"]
```

如果违反了借用规则：

```rust
fn main() {
    let a = String::from("str1");
    let rc = RefCell::new(a);
    let mut_ra1 = rc.borrow_mut();
    let mut_ra2 = rc.borrow_mut(); 
}
```

上面的代码片段能够通过编译，但是运行会抛出错误：

```
thread 'main' panicked at src/main.rs:68:22:
already borrowed: BorrowMutError
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 结合 `Rc<T>` 和 `RefCell<T>` 来拥有多个可变数据所有者

Rc 只能使用不可变引用且允许对相同数据有多个所有者，而 RefCell 具有内部可变性。如果有一个存储了多个
RefCell 的 Rc 就可以获得多个所有者并可以修改的值。

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
```

将 `List` 改造一下，使用 `Rc<RefCell<i32>>` 替换 `i32`。

```rust
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() = 15;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

运行上面的代码得到结果：

```
a before = Cons(RefCell { value: 5 }, Nil)
b before = Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Nil))
c before = Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Nil))
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

发现 value 被修改了，表面上不可变的 List 通过使用 RefCell 改变了内部的值。