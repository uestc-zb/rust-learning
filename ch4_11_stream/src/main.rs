fn main() {
    println!("Hello, world!");
}

// // 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
// async fn blocks() {
//     let my_string = "foo".to_string();

//     let future_one = async {
//         // ...
//         println!("{my_string}");
//     };

//     let future_two = async {
//         // ...
//         println!("{my_string}");
//     };

//     // 运行两个 Future 直到完成
//     let ((), ()) = futures::join!(future_one, future_two);
// }



// // 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
// // 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
// fn move_block() -> impl Future<Output = ()> {
//     let my_string = "foo".to_string();
//     async move {
//         // ...
//         println!("{my_string}");
//     }
// }

// trait Stream {
//     // Stream生成的值的类型
//     type Item;

//     // 尝试去解析Stream中的下一个值,
//     // 若无数据，返回`Poll::Pending`, 若有数据，返回 `Poll::Ready(Some(x))`, `Stream`完成则返回 `Poll::Ready(None)`
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
//         -> Poll<Option<Self::Item>>;
// }

// async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
//     use futures::stream::StreamExt; // 引入 next
//     let mut sum = 0;
//     while let Some(item) = stream.next().await {
//         sum += item;
//     }
//     sum
// }

// async fn sum_with_try_next(
//     mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
// ) -> Result<i32, io::Error> {
//     use futures::stream::TryStreamExt; // 引入 try_next
//     let mut sum = 0;
//     while let Some(item) = stream.try_next().await? {
//         sum += item;
//     }
//     Ok(sum)
// }
