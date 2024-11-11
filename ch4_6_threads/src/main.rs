// // // // // // use std::thread;
// // // // // // use std::time::Duration;

// // // // // // fn main() {
// // // // // //     let handle = thread::spawn(|| {
// // // // // //         for i in 1..10 {
// // // // // //             println!("hi number {} from the spawned thread!", i);
// // // // // //             thread::sleep(Duration::from_millis(1));
// // // // // //         }
// // // // // //     });

// // // // // //     handle.join().unwrap();

// // // // // //     for i in 1..5 {
// // // // // //         println!("hi number {} from the main thread!", i);
// // // // // //         thread::sleep(Duration::from_millis(1));
// // // // // //     }
// // // // // // }

// // // // // // use std::thread;

// // // // // // fn main() {
// // // // // //     let v = vec![1, 2, 3];

// // // // // //     let handle = thread::spawn(move || {
// // // // // //         println!("Here's a vector: {:?}", v);
// // // // // //     });

// // // // // //     handle.join().unwrap();

// // // // // //     // 下面代码会报错borrow of moved value: `v`
// // // // // //     // println!("{:?}",v);
// // // // // // }

// // // // // use std::thread;
// // // // // use std::time::Duration;
// // // // // fn main() {
// // // // //     // 创建一个线程A
// // // // //     let new_thread = thread::spawn(move || {
// // // // //         // 再创建一个线程B
// // // // //         thread::spawn(move || {
// // // // //             loop {
// // // // //                 println!("I am a new thread.");
// // // // //             }
// // // // //         })
// // // // //     });

// // // // //     // 等待新创建的线程执行完成
// // // // //     new_thread.join().unwrap();
// // // // //     println!("Child thread is finish!");

// // // // //     // 睡眠一段时间，看子线程创建的子线程是否还在运行
// // // // //     thread::sleep(Duration::from_millis(100));
// // // // // }

// // // // use std::sync::{Arc, Barrier};
// // // // use std::thread;

// // // // fn main() {
// // // //     let mut handles = Vec::with_capacity(6);
// // // //     let barrier = Arc::new(Barrier::new(6));

// // // //     for _ in 0..6 {
// // // //         let b = barrier.clone();
// // // //         handles.push(thread::spawn(move|| {
// // // //             println!("before wait");
// // // //             b.wait();
// // // //             println!("after wait");
// // // //         }));
// // // //     }

// // // //     for handle in handles {
// // // //         handle.join().unwrap();
// // // //     }
// // // // }

// // // use std::cell::RefCell;

// // // struct Foo;
// // // impl Foo {
// // //     thread_local! {
// // //         static FOO: RefCell<usize> = RefCell::new(0);
// // //     }
// // // }

// // // fn main() {
// // //     Foo::FOO.with(|x| println!("{:?}", x));
// // // }

// // use thread_local::ThreadLocal;
// // use std::sync::Arc;
// // use std::cell::Cell;
// // use std::thread;
// // fn main() {

// //     let tls = Arc::new(ThreadLocal::new());
// //     let mut v = vec![];
// //     // 创建多个线程
// //     for _ in 0..5 {
// //         let tls2 = tls.clone();
// //         let handle = thread::spawn(move || {
// //             // 将计数器加1
// //             // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
// //             // 这只能在线程退出后发生，因此不会导致任何竞争条件
// //             let cell = tls2.get_or(|| Cell::new(0));
// //             cell.set(cell.get() + 1);
// //         });
// //         v.push(handle);
// //     }
// //     for handle in v {
// //         handle.join().unwrap();
// //     }
// //     // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
// //     let tls = Arc::try_unwrap(tls).unwrap();
// //     let total = tls.into_iter().fold(0, |x, y| {
// //         // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
// //         // 因为一些线程已退出，并且其他线程会回收退出线程的对象
// //         println!("x: {}, y: {}", x, y.get());
// //         x + y.get()
// //     });
    
// // }
// // // 和为5
// // assert_eq!(total, 5);

// use std::thread;
// use std::sync::{Arc, Mutex, Condvar};

// fn main() {
//     let pair = Arc::new((Mutex::new(false), Condvar::new()));
//     let pair2 = pair.clone();

//     thread::spawn(move|| {
//         let (lock, cvar) = &*pair2;
//         let mut started = lock.lock().unwrap();
//         println!("changing started");
//         *started = true;
//         cvar.notify_one();
//     });

//     let (lock, cvar) = &*pair;
//     let mut started = lock.lock().unwrap();
//     while !*started {
//         started = cvar.wait(started).unwrap();
//     }

//     println!("started changed");
// }

use std::thread;
use std::sync::Once;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}