// // // use std::ops::Sub; // 提供了减法操作的实现。
// // // use std::sync::atomic::{AtomicU64, Ordering}; // 提供了原子操作和内存顺序保证。
// // // use std::thread::{self, JoinHandle}; // 提供了线程创建和管理的功能。
// // // use std::time::Instant; // Instant 是一个用于测量时间的类型

// // // const N_TIMES: u64 = 10000000; // 定义了一个常量 N_TIMES，表示每个线程将执行的循环次数。
// // // const N_THREADS: usize = 10; // 定义了一个常量 N_THREADS，表示将要创建的线程数量。
// // // // 定义了一个全局的 AtomicU64 类型的静态变量 R，并初始化为 0。
// // // static R: AtomicU64 = AtomicU64::new(0); 

// // // // 定义了一个函数 add_n_times，它接受一个 u64 类型的参数 n，
// // // // 并返回一个 JoinHandle。这个函数创建了一个新线程，
// // // // 该线程将执行 n 次 fetch_add 操作，每次将 R 的值增加 1。
// // // fn add_n_times(n: u64) -> JoinHandle<()> {
// // //     thread::spawn(move || {
// // //         for _ in 0..n {
// // //             R.fetch_add(1, Ordering::Relaxed);
// // //         }
// // //     })
// // // }

// // // fn main() {
// // //     let s = Instant::now();
// // //     let mut threads = Vec::with_capacity(N_THREADS);

// // //     for _ in 0..N_THREADS {
// // //         threads.push(add_n_times(N_TIMES));
// // //     }

// // //     for thread in threads {
// // //         thread.join().unwrap();
// // //     }

// // //     assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
// // //     println!("{:?}",Instant::now().sub(s));
// // //     println!("{}", R.load(Ordering::Relaxed));
// // // }

// // use std::sync::Mutex;
// // use std::sync::atomic::{Ordering, AtomicU64};

// // #[derive(Debug)]
// // struct Counter {
// //     count: u64
// // }

// // fn main() {
// //     let n = Mutex::new(Counter {
// //         count: 0
// //     });

// //     {
// //         n.lock().unwrap().count += 1;
// //     }
// //     let num = n.lock().unwrap();
// //     println!("{}", num.count);
// //     let n = AtomicU64::new(0);

// //     n.fetch_add(0, Ordering::Relaxed);
// //     println!("{}", n.load(Ordering::Relaxed));
// // }

// use std::thread::{self, JoinHandle};
// use std::sync::atomic::{Ordering, AtomicBool};

// static mut DATA: u64 = 0;
// static READY: AtomicBool = AtomicBool::new(false);

// fn reset() {
//     unsafe {
//         DATA = 0;
//     }
//     READY.store(false, Ordering::Relaxed);
// }

// fn producer() -> JoinHandle<()> {
//     thread::spawn(move || {
//         unsafe {
//             DATA = 100;                                 // A
//         }
//         READY.store(true, Ordering::Release);           // B: 内存屏障 ↑
//     })
// }

// fn consumer() -> JoinHandle<()> {
//     thread::spawn(move || {
//         while !READY.load(Ordering::Acquire) {}         // C: 内存屏障 ↓

//         assert_eq!(100, unsafe { DATA });               // D
//     })
// }


// fn main() {
//     loop {
//         reset();

//         let t_producer = producer();
//         let t_consumer = consumer();

//         t_producer.join().unwrap();
//         t_consumer.join().unwrap();
//     }
// }

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}