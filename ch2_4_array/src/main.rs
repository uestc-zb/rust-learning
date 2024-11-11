use std::io;
#[allow(unused_variables)]

fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [9, 8, 7, 6, 5];

    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    let array:[String; 8] = std::array::from_fn(|_i| String::from("rust is good"));
    println!("{:#?}", array);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}
