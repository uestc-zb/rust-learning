// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());

// }



// use std::fmt::Display;

// fn create_and_print<T>() where T: From<i32> + Display {
//     let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
//     println!("a is: {}", a);
// }

// fn main() {
//     create_and_print::<i64>();
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }



// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(arr);
// }

// const fn add(a: usize, b: usize) -> usize {
//     a + b
// }

// const RESULT: usize = add(5, 10);

// fn main() {
//     println!("The result is: {}", RESULT);
// }

struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}

fn main() {
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> {
        data: [0; SIZE],
    };
    println!("Buffer size: {} bytes", buffer.data.len());
}