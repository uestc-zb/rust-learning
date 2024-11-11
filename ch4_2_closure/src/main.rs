// fn main() {
//     let x = 1;
//     let sum = |y| x + y;
    
//     assert_eq!(3, sum(2));
    
//     let sum = |x: i32, y: i32| -> i32 {
//         x + y
//     };    
//     let v = sum(1, 2);
// }

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     query: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(query: T) -> Cacher<T> {
//         Cacher {
//             query,
//             value: None,
//         }
//     }

//     // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;

//     let y = 4;

//     assert!(equal_to_x(y));
// }

// fn fn_once<F>(func: F)
// where
//     F: FnOnce(usize) -> bool + Copy,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     // let x = vec![1, 2, 3];
//     // fn_once(|z|{z == x.len()})
//     use std::thread;
//     let v = vec![1, 2, 3];
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });
//     handle.join().unwrap();

// }

// fn main() {
//     let mut s = String::new();

//     let mut update_string =  |str| s.push_str(str);
//     update_string("hello");

//     println!("{:?}",s);
// }


// fn main() {
//     let mut s = String::new();

//     let update_string =  |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}",s);
// }

// fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
//     f("hello")
// }

// fn main() {
//     let mut s = "String::new();".to_string();

//     let update_string =  |str| println!("{},{}",s, str);

//     exec(update_string);

//     println!("{:?}",s);
// }

// fn exec<'a, F: Fn(String)>(f: F)  {
//     f("hello".to_string())
// }

// fn main() {
//     let s = String::new();

//     let update_string =  move || println!("{}",s);

//     exec(update_string);
// }

// fn exec<F: FnOnce()>(f: F)  {
//     f()
// }

// fn main() {
//     let s = String::new();

//     let update_string =  || println!("{}",s);

//     exec(update_string);
//     exec1(update_string);
//     exec2(update_string);
// }

// fn exec<F: FnOnce()>(f: F)  {
//     f()
// }

// fn exec1<F: FnMut()>(mut f: F)  {
//     f()
// }

// fn exec2<F: Fn()>(f: F)  {
//     f()
// }

fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);

}

fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) {
    f("hello");
}

fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
