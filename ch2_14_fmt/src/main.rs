// // fn main() {
// //     println!("Hello, world!");
// //     println!("Hello");                 // => "Hello"
// //     println!("Hello, {}!", "world");   // => "Hello, world!"
// //     println!("The number is {}", 1);   // => "The number is 1"
// //     println!("{:?}", (3, 4));          // => "(3, 4)"
// //     println!("{value}", value=4);      // => "4"
// //     println!("{} {}", 1, 2);           // => "1 2"
// //     println!("{:04}", 42);             // => "0042" with leading zeros

// // }

// // fn main() {
// //     let s = "hello";
// //     println!("{}, world", s);
// //     let s1 = format!("{}, world", s);
// //     print!("{}", s1);
// //     print!("{}\n", "!");
// //     eprintln!("Error: Could not complete task")

// // }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8
// }

// fn main() {
//     let i = 3.1415926;
//     let s = String::from("hello");
//     let v = vec![1, 2, 3];
//     let p = Person{name: "sunface".to_string(), age: 18};
//     println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);

//     let i = 3.1415926;
//     let s = String::from("hello");
//     let v = vec![1, 2, 3];
//     let p = Person {
//         name: "sunface".to_string(),
//         age: 18,
//     };
//     println!("{}, {}, {:?}, {:?}", i, s, v, p);

// }

struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
// fn main() {
//     let p = Person {
//         name: "sunface".to_string(),
//         age: 18,
//     };
//     println!("{}", p);
// }

struct Array(Vec<i32>);

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}
// fn main() {
//     let arr = Array(vec![1, 2, 3]);
//     println!("{}", arr);
// }

// fn main() {
//     println!("{}{}", 1, 2); // =>"12"
//     println!("{1}{0}", 1, 2); // =>"21"
//     // => Alice, this is Bob. Bob, this is Alice
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//     println!("{1}{}{0}{}", 1, 2); // => 2112

// }
// fn main() {
//     let v = 3.1415926;
//     // Display => 3.14
//     println!("{:.2}", v);
//     // Debug => 3.14
//     println!("{:.2?}", v);
// }


// fn main() {
//     //-----------------------------------
//     // 以下全部输出 "Hello x    !"
//     // 为"x"后面填充空格，补齐宽度5
//     println!("Hello {:5}!", "x");
//     // 使用参数5来指定宽度
//     println!("Hello {:1$}!", "x", 5);
//     // 使用x作为占位符输出内容，同时使用5作为宽度
//     println!("Hello {1:0$}!", 5, "x");
//     // 使用有名称的参数作为宽度
//     println!("Hello {:width$}!", "x", width = 5);
//     //-----------------------------------

//     // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
//     println!("Hello {:1$}!{}", "x", 5);
// }

// fn main() {
//     // 宽度是5 => Hello     5!
//     println!("Hello {:5}!", 5);
//     // 显式的输出正号 => Hello +5!
//     println!("Hello {:+}!", 5);
//     // 宽度5，使用0进行填充 => Hello 00005!
//     println!("Hello {:05}!", 5);
//     // 负号也要占用一位宽度 => Hello -0005!
//     println!("Hello {:05}!", -5);
// }

// fn main() {
//     // 以下全部都会补齐5个字符的长度
//     // 左对齐 => Hello x    !
//     println!("Hello {:<5}!", "x");
//     // 右对齐 => Hello     x!
//     println!("Hello {:>5}!", "x");
//     // 居中对齐 => Hello   x  !
//     println!("Hello {:^5}!", "x");

//     // 对齐并使用指定符号填充 => Hello x&&&&!
//     // 指定符号填充的前提条件是必须有对齐字符
//     println!("Hello {:&<5}!", "x");
// }

fn main() {
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050
    println!(" Hello \"{{World}}\" ");


}

