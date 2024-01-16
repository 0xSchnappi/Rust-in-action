# Rust Base

## 浮点数陷阱()

如果不谨慎处理，浮点数类型（例如f32和f64）可能会导致严重的问题。这类问题（至少）是由以下两个原因所导致的。

1. 浮点数类型的值通常表示的是某个数字的近似值.在计算机中,浮点数实际是使用二进制来实现的,但是我们通常想用十进制来完成浮点数运算,这种不匹配性导致了歧义的产生。我们所关心的许多数值，比如说0.1，是无法用二进制来精确表示的;比如1/3，用十进制数字系统是无法精确表示的。
2. 浮点数类型只具有部分等价关系（partial equivalence relation)，这种关系被编码到Rust的类型系统中了。f32和f64类型只实现了 `std::cmp::PartialEq`

要想避免出现这类问题，下面给出两个指导方针。

1. 避免测试浮点数的相等性。
2. 如果结果可能是在数学上未定义时，要谨慎对待。

为了说明这个问题，运行下面这段代码。虽然这个被求值的表达式所对应的数学表示(0.1 + 0.2 = 0.3)是没有问题的，但是这段代码在大多数系统中运行的时候都会崩溃：

```rust
fn main () {
  assert!(0.1 + 0.2 == 0.3)
}
```

但不是所有的浮点数类型都会这样

```rust
fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);    //<1>
    assert!(xyz.0 + xyz.1 == xyz.2);    //<2>
}
```

次程序执行后，揭示了错误的原因

```shell
abc (f32)
   0.1 + 0.2: 3e99999a
         0.3: 3e99999a

xyz (f64)
   0.1 + 0.2: 3fd3333333333334
         0.3: 3fd3333333333333

thread 'main' panicked at ch2-add-floats.rs:16:5:
assertion failed: xyz.0 + xyz.1 == xyz.2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

一般来说，测试一个数学运算的结果是否会落在其真实数学结果的一个可接受范围内是比较安全的，这个范围通常被称为机器极小值（epsilon)或最小单元取整数

Rust包含一些容错性，使得浮点数的比较运算能得到预期的结果。这些容错性源自对f32::EPSILON和f64::EPSILON的定义。下面这段代码所展示的，与Rust底层实现中所做的事情是比较接近的：

```rust
fn main () {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
}
```

Rust编译器实际上把比较运算委托给CPU了，浮点数是通过芯片内的定制硬件来实现，非法的或者未定义的运算会触发一个CPU的异常。

某些运算会产生在数学上未定义的结果，比如求一个负数的平方根（-42.0.sqrt())，这就带来了特殊的问题，在浮点数类型中有一个“非数字"（Not a Number)的值（用Rust的语法表示为值NAN)，专门用于处理这类情况。

值NAN会污染其他的数值。NAN参与的运算大部分会返回NAN。另一个需要注意的事情是，根据定义，一个NAN的值并不等于另一个NAN的值。下面这个小例子总是会崩溃：

```rust
fn main() {
  let x = (-42.0_f32).sqrt();
  assert_eq!(x, x);
}
```

要进行防御性编程（Defensive programming)，可以使用is_nan()和is_finite()方法。引发崩溃，而不是默默带着数学错误继续执行，这让你可以在接近导致问题的地方进行调试。

```rust
fn main() {
  let x: f32 = 1.0 / 0.0;
  assert!(x.is_finite());
}
```
