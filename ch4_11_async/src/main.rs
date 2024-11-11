
// // // // // fn get_two_sites() {
// // // //     //     // 创建两个新线程执行任务
// // // //     //     let thread_one = thread::spawn(|| download("https://course.rs"));
// // // //     //     let thread_two = thread::spawn(|| download("https://fancy.rs"));
    
// // // //     //     // 等待两个线程的完成
// // // //     //     thread_one.join().expect("thread one panicked");
// // // //     //     thread_two.join().expect("thread two panicked");
// // // //     // }

// // // // // async fn get_two_sites_async() {
// // // //     //     // 创建两个不同的`future`，你可以把`future`理解为未来某个时刻会被执行的计划任务
// // // //     //     // 当两个`future`被同时执行后，它们将并发的去下载目标页面
// // // //     //     let future_one = download_async("https://www.foo.com");
// // // // //     let future_two = download_async("https://www.bar.com");

// // // // //     // 同时运行两个`future`，直至完成
// // // // //     join!(future_one, future_two);
// // // // // }

// // // // fn main() {
// // // //     do_something();
// // // // }

// // // // async fn do_something() {
// // // //     println!("go go go !");
// // // // }

// // // // `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
// // // // 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行。
// // // use futures::executor::block_on;

// // // async fn hello_world() {
// // //     println!("hello, world!");
// // // }

// // // fn main() {
// // //     let future = hello_world(); // 返回一个Future, 因此不会打印任何输出
// // //     block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出
// // // }


// // use futures::executor::block_on;

// // async fn hello_world() {
// //     hello_cat().await;
// //     println!("hello, world!");
// // }

// // async fn hello_cat() {
// //     println!("hello, kitty!");
// // }
// // fn main() {
// //     let future = hello_world();
// //     block_on(future);
// // }

// use futures::executor::block_on;

// struct Song {
//     author: String,
//     name: String,
// }

// async fn learn_song() -> Song {
//     Song {
//         author: "周杰伦".to_string(),
//         name: String::from("《菊花台》"),
//     }
// }

// async fn sing_song(song: Song) {
//     println!(
//         "给大家献上一首{}的{} ~ {}",
//         song.author, song.name, "菊花残，满地伤~ ~"
//     );
// }

// async fn dance() {
//     println!("唱到情深处，身体不由自主的动了起来~ ~");
// }

// fn main() {
//     let song = block_on(learn_song());
//     block_on(sing_song(song));
//     block_on(dance());
// }

use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}