# 集合

## vector

```rust
fn main() {
	let mut v1: Vec<i32> = Vec::new();
	v1.push(1);
	v1.push(2);
	v1.push(3);

	for i in 0..2 {
		println!("{} ", v1[i]);
	}

	for i in 1..4 {
		if let Some(num) = v.get(i) {
			println!("{} ", num);
		}
	}

	for i in &mut v {
		*i += 50
	}
}
```

使用 `Vec::new()` 或 `vec![]` 新建 vector。

使用下标或者 `get` 方法访问 vector 元素，`get` 方法返回一个 Option。

使用 `for-each` 遍历 vector

## String

## HashMap

```rust
fn main() {
	use std::collections::Hashmap;

	let mut map = Hashmap::new();
	map.insert(1, "hello");

	let v1 = map.entry(1).or_insert("hello2");
	let v2 = map.entry(2).or_insert("hashmap");
	println!("{:?}", map);

	*v1 = String::from("hashmap2");

	for (k, v) in map {
		println!("{k}:{v}");
	}
}
```

