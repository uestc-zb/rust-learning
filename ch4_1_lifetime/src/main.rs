// #[derive(Debug)]
// struct Foo;

// impl Foo {
//     fn mutate_and_share(&mut self) -> &Self {
//         &*self
//     }
//     fn share(&self) {}
// }

// fn main() {
//     let mut foo = Foo;
//     let loan = foo.mutate_and_share();
//     foo.share();
//     println!("{:?}", loan);
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::HashMap;
//     use std::hash::Hash;
//     fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
//     where
//         K: Clone + Eq + Hash,
//         V: Default,
//     {
//         match map.get_mut(&key) {
//             Some(value) => value,
//             None => {
//                 map.insert(key.clone(), V::default());
//                 map.get_mut(&key).unwrap()
//             }
//         }
//     }
// }

// // fn f<'a, T>(x: *const T) -> &'a T {
// //     unsafe {
// //         &*x
// //     }
// // }

// struct DoubleRef<'a,'b:'a, T> {
//     r: &'a T,
//     s: &'b T
// }

// struct Ref<'a, T: 'a> {
//     r: &'a T
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a: 'b, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let mut s = String::from("hello");
 
//      let r1 = &s;
//      let r2 = &s;
//      println!("{} and {}", r1, r2);
//      // 新编译器中，r1,r2作用域在这里结束
 
//      let r3 = &mut s;
//      println!("{}", r3);
//  }

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn move_to(&mut self, x: i32, y: i32) {
//         self.x = x;
//         self.y = y;
//     }
// }

// fn main() {
//     let mut p = Point { x: 0, y: 0 };
//     let r = &mut p;
//     let rr: &Point = &*r;

//     println!("{:?}", rr);
//     r.move_to(10, 10);
//     println!("{:?}", r);
// }

// struct Interface<'b, 'a: 'b> {
//     manager: &'b mut Manager<'a>
// }

// impl<'b, 'a: 'b> Interface<'b, 'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }

// struct Manager<'a> {
//     text: &'a str
// }

// struct List<'a> {
//     manager: Manager<'a>,
// }

// impl<'a> List<'a> {
//     pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
//     where 'a:'b {
//         Interface {
//             manager: &mut self.manager
//         }
//     }
    
// }


// fn main() {

//     let mut list = List {
//         manager: Manager {
//             text: "hello"
//         }
//     };

//     list.get_interface().noop();

//     println!("Interface should be dropped here and the borrow released");

//     // 下面的调用可以通过，因为Interface的生命周期不需要跟list一样长
//     use_list(&list);
// }

// fn use_list(list: &List) {
//     println!("{}", list.manager.text);
// }


fn main() {
    let mark_twain: &str = "Samuel Clemens";
    print_author(mark_twain);
  }
  fn print_author(author: &'static str) {
    println!("{}", author);
  }








