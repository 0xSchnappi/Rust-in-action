# Rust Characteristic

## 安全性

- 悬垂指针
    > 引用了在程序运行过程中已经变为无效的数据
    ```rust
    #[derive(Debug)]    // <1>
    enum Cereal {       // <2>
        Barley, Millet, Rice,
        Rye, Spelt, Wheat,
    }

    fn main() {
        let mut grains: Vec<Cereal> = vec![];   // <3>
        grains.push(Cereal::Rye);               // <4>
        drop(grains);                           // <5>

        println!("{:?}", grains);               // <6>
    }
    ```
- 数据竞争
    > 由于外部因素的变化，无法确定程序在每次运行时的行为
    ```rust
    use std::thread;                          // <1>

    fn main() {
        let mut data = 100;

        thread::spawn(|| { data = 500; });    // <2>
        thread::spawn(|| { data = 1000; });   // <2>

        println!("{}", data);
    }
    ```
- 缓冲区溢出
    > 例如一个只有6个元素的数组，试图访问其中的第12个元素
    ```rust
    fn main() {
        let fruit = vec!['🥝', '🍌', '🍇'];

        let buffer_overflow = fruit[4];    // <1>

        assert_eq!(buffer_overflow, '🍉')  // <2>
    }
    ```
- 迭代器失效
    > 在迭代的过程中，迭代器中值被更改而导致的问题
    ```rust
    fn main() {
        let mut letters = vec![            // <1>
            "a", "b", "c"
        ];

        for letter in letters {
            println!("{}", letter);
            letters.push(letter.clone());  // <2>
        }
    }

    ```