// // // // // fn main() {
// // // // //     println!("Hello, world!");
// // // // // }
// // // // fn main() {
// // // //     let mut num = 5;

// // // //     let r1 = &num as *const i32;

// // // //     unsafe {
// // // //         println!("r1 is: {}", *r1);
// // // //     }

// // // //     let mut num = 5;

// // // //     let r1 = &num as *const i32;
// // // //     let r2 = &mut num as *mut i32;

// // // // }

// // // use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// // // // 获取字符串的内存地址和长度
// // // fn get_memory_location() -> (usize, usize) {
// // //   let string = "Hello World!";
// // //   let pointer = string.as_ptr() as usize;
// // //   let length = string.len();
// // //   (pointer, length)
// // // }

// // // // 在指定的内存地址读取字符串
// // // fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
// // //   unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
// // // }

// // // fn main() {
// // //   let (pointer, length) = get_memory_location();
// // //   let message = get_str_at_location(pointer, length);
// // //   println!(
// // //     "The {} bytes at 0x{:X} stored: {}",
// // //     length, pointer, message
// // //   );
// // //   // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
// // //   // let message = get_str_at_location(1000, 10);
// // //   let a: Box<i32> = Box::new(10);
// // //     // 需要先解引用a
// // //     let b: *const i32 = &*a;
// // //     // 使用 into_raw 来创建
// // //     let c: *const i32 = Box::into_raw(a);

// // // }

// // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// //     let len = slice.len();

// //     assert!(mid <= len);

// //     (&mut slice[..mid], &mut slice[mid..])
// // }

// // fn main() {
// //     let mut v = vec![1, 2, 3, 4, 5, 6];

// //     let r = &mut v[..];

// //     let (a, b) = split_at_mut(r, 3);

// //     assert_eq!(a, &mut [1, 2, 3]);
// //     assert_eq!(b, &mut [4, 5, 6]);
// // }

// use std::slice;

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];

//     let (a, b) = split_at_mut(r, 3);

//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}