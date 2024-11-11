// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
  
//     if a < (b as i32) {
//       println!("Ten is less than one hundred.");
//     }

//     let a = i8::MAX;
//     println!("{}",a);

// }

// fn main() {
//     let a = 3.1 as i8;
//     let b = 100_i8 as i32;
//     let c = 'a' as u8; // 将字符'a'转换为整数，97
 
//     println!("{},{},{}",a,b,c);

//     let mut values: [i32; 2] = [1, 2];
//     let p1: *mut i32 = values.as_mut_ptr();
//     let first_address = p1 as usize; // 将p1内存地址转换为一个整数
//     let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
//     let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
//     unsafe {
//         *p2 += 1;
//     }
//     assert_eq!(values[1], 3);

// }

use std::convert::TryInto;

// fn main() {
//    let a: u8 = 10;
//    let b: u16 = 1500;

//    let b_: u8 = b.try_into().unwrap();

//    if a < b_ {
//      println!("Ten is less than one hundred.");
//    }
// }

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() {
    let t: &mut i32 = &mut 0;
    // foo(t);
}







