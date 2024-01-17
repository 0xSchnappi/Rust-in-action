use std::borrow::Cow; // 一种智能指针类型，能够从其指针位置读取数据而无须先复制它
use std::ffi::CStr;
use std::os::raw::c_char; // i8的别名

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
fn main() {
    let a1 = 42;
    let b1 = &B;
    let c1 = &C;

    println!("a: {}, b: {:p}, c:{:p}", a1, b1, c1);

    println!("{}", type_of(B));

    let b2: String;
    let c2: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b2 = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c2 = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("b: {}, c: {}", b2, c2);

    // 解引用一个指针
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {}, a_ptr: {:p}, a_addr: 0x{:x}", a, a_ptr, a_addr + 7)

    let ttt = "kjhfhs";
}

// fn main() {
//     let a:i64 = 42;
//     let a_ptr = &a as *const i64;

//     println!("a: {}, a_ptr: {:p}", a, a_ptr);

//     // 解裸指针
//     println!("a_ptr value: {}", unsafe { *a_ptr });

// }
