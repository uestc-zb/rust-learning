// // // // // // // // // const MAX_ID: usize =  usize::MAX / 2;
// // // // // // // // // fn main() {
// // // // // // // // //    println!("用户ID允许的最大值是{}",MAX_ID);
// // // // // // // // // }

// // // // // // // // static mut REQUEST_RECV: usize = 0;
// // // // // // // // fn main() {
// // // // // // // //    unsafe {
// // // // // // // //         REQUEST_RECV += 1;
// // // // // // // //         assert_eq!(REQUEST_RECV, 1);
// // // // // // // //    }
// // // // // // // // }

// // // // // // // use std::sync::atomic::{AtomicUsize, Ordering};
// // // // // // // static REQUEST_RECV: AtomicUsize  = AtomicUsize::new(0);
// // // // // // // fn main() {
// // // // // // //     for _ in 0..100 {
// // // // // // //         REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
// // // // // // //     }

// // // // // // //     println!("当前用户请求数{:?}",REQUEST_RECV);
// // // // // // // }

// // // // // // use std::sync::atomic::{Ordering, AtomicUsize};

// // // // // // struct Factory{
// // // // // //     factory_id: usize,
// // // // // // }

// // // // // // static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
// // // // // // const MAX_ID: usize = usize::MAX / 2;

// // // // // // fn generate_id()->usize{
// // // // // //     // 检查两次溢出，否则直接加一可能导致溢出
// // // // // //     let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
// // // // // //     if current_val > MAX_ID{
// // // // // //         panic!("Factory ids overflowed");
// // // // // //     }
// // // // // //     GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
// // // // // //     let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
// // // // // //     if next_id > MAX_ID{
// // // // // //         panic!("Factory ids overflowed");
// // // // // //     }
// // // // // //     next_id
// // // // // // }

// // // // // // impl Factory{
// // // // // //     fn new()->Self{
// // // // // //         Self{
// // // // // //             factory_id: generate_id()
// // // // // //         }
// // // // // //     }
// // // // // // }

// // // // // // use std::sync::Mutex;
// // // // // // static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));

// // // // // // fn main() {
// // // // // //     let v = NAMES.lock().unwrap();
// // // // // //     println!("{}",v);
// // // // // // }

// // // // // use std::sync::Mutex;
// // // // // use lazy_static::lazy_static;
// // // // // lazy_static! {
// // // // //     static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
// // // // // }

// // // // // fn main() {
// // // // //     let mut v = NAMES.lock().unwrap();
// // // // //     v.push_str(", Myth");
// // // // //     println!("{}",v);
// // // // // }

// // // // use lazy_static::lazy_static;
// // // // use std::collections::HashMap;

// // // // lazy_static! {
// // // //     static ref HASHMAP: HashMap<u32, &'static str> = {
// // // //         let mut m = HashMap::new();
// // // //         m.insert(0, "foo");
// // // //         m.insert(1, "bar");
// // // //         m.insert(2, "baz");
// // // //         m
// // // //     };
// // // // }

// // // // fn main() {
// // // //     // 首次访问`HASHMAP`的同时对其进行初始化
// // // //     println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

// // // //     // 后续的访问仅仅获取值，再不会进行任何初始化操作
// // // //     println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
// // // // }

// // // #[derive(Debug)]
// // // struct Config {
// // //     a: String,
// // //     b: String,
// // // }
// // // static mut CONFIG: Option<&mut Config> = None;

// // // fn main() {
// // //     unsafe {
// // //         CONFIG = Some(&mut Config {
// // //             a: "A".to_string(),
// // //             b: "B".to_string(),
// // //         });

// // //         println!("{:?}", CONFIG)
// // //     }
// // // }

// // #[derive(Debug)]
// // struct Config {
// //     a: String,
// //     b: String
// // }
// // static mut CONFIG: Option<&mut Config> = None;

// // fn main() {
// //     let c = Box::new(Config {
// //         a: "A".to_string(),
// //         b: "B".to_string(),
// //     });

// //     unsafe {
// //         // 将`c`从内存中泄漏，变成`'static`生命周期
// //         CONFIG = Some(Box::leak(c));
// //         println!("{:?}", CONFIG);
// //     }
// // }

// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut CONFIG: Option<&mut Config> = None;

// fn init() -> Option<&'static mut Config> {
//     Some(&mut Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     })
// }


// fn main() {
//     unsafe {
//         CONFIG = init();

//         println!("{:?}", CONFIG)
//     }
// }

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}


fn main() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}