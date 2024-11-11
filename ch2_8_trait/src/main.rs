// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub struct Post {
//     pub title: String, // 标题
//     pub author: String, // 作者
//     pub content: String, // 内容
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

// // pub fn notify(item: &impl Summary) {
// //     println!("Breaking news! {}", item.summarize());
// // }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn returns_summarizable() -> impl Summary {
//     Weibo {
//         username: String::from("sunface"),
//         content: String::from(
//             "m1 max太厉害了，电脑再也不会卡",
//         )
//     }
// }



// pub struct Weibo {
//     pub username: String,
//     pub content: String
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
//     let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

//     println!("{}",post.summarize());
//     println!("{}",weibo.summarize());
// }


// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// use std::convert::TryInto;

// fn main() {
//   let a: i32 = 10;
//   let b: u16 = 100;

//   let b_ = b.try_into()
//             .unwrap();

//   if a < b_ {
//     println!("Ten is less than one hundred.");
//   }
// }

// use std::ops::Add;

// // 为Point结构体派生Debug特征，用于格式化输出
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point{
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point{x: 1.1f32, y: 1.1f32};
//     let p2 = Point{x: 2.1f32, y: 2.1f32};
//     println!("{:?}", add(p1, p2));

//     let p3 = Point{x: 1i32, y: 1i32};
//     let p4 = Point{x: 2i32, y: 2i32};
//     println!("{:?}", add(p3, p4));
// }


#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug,PartialEq)]
enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl Display for FileState {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     match *self {
         FileState::Open => write!(f, "OPEN"),
         FileState::Closed => write!(f, "CLOSED"),
     }
   }
}

impl Display for File {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{} ({})>",
             self.name, self.state)
   }
}

impl File {
  fn new(name: &str) -> File {
    File {
        name: String::from(name),
        data: Vec::new(),
        state: FileState::Closed,
    }
  }
}

fn main() {
  let f6 = File::new("f6.txt");
  //...
  println!("{:?}", f6);
  println!("{}", f6);
}
