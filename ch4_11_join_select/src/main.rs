fn main() {
    println!("Hello, world!");
}

// use futures::join;

// async fn enjoy_book_and_music() -> (Book, Music) {
//     let book_fut = enjoy_book();
//     let music_fut = enjoy_music();
//     join!(book_fut, music_fut)
// }

// use futures::try_join;

// async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
// async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

// async fn get_book_and_music() -> Result<(Book, Music), String> {
//     let book_fut = get_book();
//     let music_fut = get_music();
//     try_join!(book_fut, music_fut)
// }

// use futures::{
//     future::TryFutureExt,
//     try_join,
// };

// async fn get_book() -> Result<Book, ()> { /* ... */ Ok(Book) }
// async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

// async fn get_book_and_music() -> Result<(Book, Music), String> {
//     let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
//     let music_fut = get_music();
//     try_join!(book_fut, music_fut)
// }

// use futures::{
//     future::FutureExt, // for `.fuse()`
//     pin_mut,
//     select,
// };

// async fn task_one() { /* ... */ }
// async fn task_two() { /* ... */ }

// async fn race_tasks() {
//     let t1 = task_one().fuse();
//     let t2 = task_two().fuse();

//     pin_mut!(t1, t2);

//     select! {
//         () = t1 => println!("任务1率先完成"),
//         () = t2 => println!("任务2率先完成"),
//     }
// }

// use futures::future;
// use futures::select;
// pub fn main() {
//     let mut a_fut = future::ready(4);
//     let mut b_fut = future::ready(6);
//     let mut total = 0;

//     loop {
//         select! {
//             a = a_fut => total += a,
//             b = b_fut => total += b,
//             complete => break,
//             default => panic!(), // 该分支永远不会运行，因为 `Future` 会先运行，然后是 `complete`
//         };
//     }
//     assert_eq!(total, 10);
// }

// use futures::{
//     future::{Fuse, FusedFuture, FutureExt},
//     stream::{FusedStream, Stream, StreamExt},
//     pin_mut,
//     select,
// };

// async fn get_new_num() -> u8 { /* ... */ 5 }

// async fn run_on_new_num(_: u8) { /* ... */ }

// async fn run_loop(
//     mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
//     starting_num: u8,
// ) {
//     let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
//     let get_new_num_fut = Fuse::terminated();
//     pin_mut!(run_on_new_num_fut, get_new_num_fut);
//     loop {
//         select! {
//             () = interval_timer.select_next_some() => {
//                 // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
//                 if get_new_num_fut.is_terminated() {
//                     get_new_num_fut.set(get_new_num().fuse());
//                 }
//             },
//             new_num = get_new_num_fut => {
//                 // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
//                 run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
//             },
//             // 运行 `run_on_new_num_fut`
//             () = run_on_new_num_fut => {},
//             // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
//             //后，执行到 `complete` 分支
//             complete => panic!("`interval_timer` completed unexpectedly"),
//         }
//     }
// }
