# 深入理解数据

## 浮点数

浮点数42.42计算机存储为01000010001010011010111000010100

| 名称             | 二进制表示              | 十进制表示的组成部分(u32) | 解码后的值         |
| ---------------- | ----------------------- | ------------------------- | ------------------ |
| 符号位(s)        | 0                       | 0                         | --1^0=1            |
| 指数位(t)        | 10000100                | 132                       | 132-127=5          |
| 尾数/有效数字(m) | 01010011010111000010100 | 2731540                   | 1.325625           |
| 底数/基数        |                         |                           | 2(浮点数以2为基数) |
| 指数偏移量       |                         |                           | 127                |

### 分离出符号位

```rust
let n: f32 = 42.42;
let n_bits: u32 = n.to_bits();
let sign_bit = n_bits >> 31;
```

### 分离出指数

```rust
let n: f32 = 42.42;
let n_bits: u32 = n.to_bits();
let exponent_ = n_bits >> 23;
let exponent_  = exponent_ & 0xff;
let exponent = (exponent_ as i32) -127
```

### 分离尾数

### 完整代码

```rust
use std::mem;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;
fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field | as bits | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = bits >> 31;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = (2_f32).powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

```
