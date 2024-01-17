
# unsafe关键字

unsafe关键字的是解决在你能确保这块代码是安全的，你可以负责这块代码的安全性，但是呆板编译器按照死板语法是无法编译通过的，在使用了unsafe关键字后，即可编译成功。

`unsafe superpowers`不安全的超能力：

1. 解引用裸指针
2. 调用不安全的函数或方法
3. 访问或修改可变静态变量
4. 实现不安全的trait
5. 访问union的字段

## 解引用裸指针

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &num as *mut i32;

unsafe {
	println!("r1 is: {}", *r1);
	println!("r2 is: {}", *r2);
}
```

> 可以在安全代码中创建裸指针，不过不能解引用裸指针和读取其指向的数据------创建一个指针不会造成任何危险，只有当访问其指向的值时才有可能遇到无效的值。

## 调用不安全的函数和方法

```rust
unsafe fn dangerous() {}

unsafe {
	dangerous();
}
```

> 调用一个没有任何操作的不安全函数dangerous函数

## 读取或修改一个可变静态变量是不安全的

```rust
static mut COUNTER: u32 = 0;
fn add_to_counter(inc: u32) {
	unsafe {
		counter += inc
	}
}

fn main() {
	add_to_counter(3);
	unsafe {
		println!("COUNTER:{}", COUNTER);
	}
}
```

> 可变静态变量通过mut定义，任何读写COUNTER的代码都必须位于unsafe块中。多个线程访问COUNTER则可能导致数据竞争，这就是为何Rust认为可变静态变量是不安全的。

## 实现不安全的trait

```rust
unsafe trait Foo {
}

unsafe impl Foo for i32 {
}

fn main() {}
```

当trait中至少有一个方法中包含编译器无法验证的不变式(invariant)(比如裸指针)时trait是不安全的。

## 访问联合体中的字段

仅适用于unsafe的最后一个操作是访问联合体中的字段。联合体主要用于和C代码中的联合体交互。
