fn main() {
    println!("Hello, world!");
}

// trait SimpleFuture {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// pub struct SocketRead<'a> {
//     socket: &'a Socket,
// }pub struct AndThenFut<FutureA, FutureB> {
//     first: Option<FutureA>,
//     second: FutureB,
// }

// impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
// where
//     FutureA: SimpleFuture<Output = ()>,
//     FutureB: SimpleFuture<Output = ()>,
// {
//     type Output = ();
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if let Some(first) = &mut self.first {
//             match first.poll(wake) {
//                 // 我们已经完成了第一个 Future， 可以将它移除， 然后准备开始运行第二个
//                 Poll::Ready(()) => self.first.take(),
//                 // 第一个 Future 还不能完成
//                 Poll::Pending => return Poll::Pending,
//             };
//         }

//         // 运行到这里，说明第一个Future已经完成，尝试去完成第二个
//         self.second.poll(wake)
//     }
// }

// trait Future {
//     type Output;
//     fn poll(
//         // 首先值得注意的地方是，`self`的类型从`&mut self`变成了`Pin<&mut Self>`:
//         self: Pin<&mut Self>,
//         // 其次将`wake: fn()` 修改为 `cx: &mut Context<'_>`:
//         cx: &mut Context<'_>,
//     ) -> Poll<Self::Output>;
// }


// impl SimpleFuture for SocketRead<'_> {
//     type Output = Vec<u8>;

//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if self.socket.has_data_to_read() {
//             // socket有数据，写入buffer中并返回
//             Poll::Ready(self.socket.read_buf())
//         } else {
//             // socket中还没数据
//             //
//             // 注册一个`wake`函数，当数据可用时，该函数会被调用，
//             // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
//             self.socket.set_readable_callback(wake);
//             Poll::Pending
//         }
//     }
// }

// 一个SimpleFuture, 它使用顺序的方式，一个接一个地运行两个Future
//
// 注意: 由于本例子用于演示，因此功能简单，`AndThenFut` 会假设两个 Future 在创建时就可用了.
// 而真实的`Andthen`允许根据第一个`Future`的输出来创建第二个`Future`，因此复杂的多。
// pub struct AndThenFut<FutureA, FutureB> {
//     first: Option<FutureA>,
//     second: FutureB,
// }

// impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
// where
//     FutureA: SimpleFuture<Output = ()>,
//     FutureB: SimpleFuture<Output = ()>,
// {
//     type Output = ();
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if let Some(first) = &mut self.first {
//             match first.poll(wake) {
//                 // 我们已经完成了第一个 Future， 可以将它移除， 然后准备开始运行第二个
//                 Poll::Ready(()) => self.first.take(),
//                 // 第一个 Future 还不能完成
//                 Poll::Pending => return Poll::Pending,
//             };
//         }

//         // 运行到这里，说明第一个Future已经完成，尝试去完成第二个
//         self.second.poll(wake)
//     }
// }

// trait Future {
//     type Output;
//     fn poll(
//         // 首先值得注意的地方是，`self`的类型从`&mut self`变成了`Pin<&mut Self>`:
//         self: Pin<&mut Self>,
//         // 其次将`wake: fn()` 修改为 `cx: &mut Context<'_>`:
//         cx: &mut Context<'_>,
//     ) -> Poll<Self::Output>;
// }
