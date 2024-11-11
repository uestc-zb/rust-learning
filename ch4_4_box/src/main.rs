// fn main() {
//     let b = foo("world");
//     println!("{}", b);
// }

// fn foo(x: &str) -> String {
//     let a = "Hello, ".to_string() + x;
//     a
// }

fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    let b = *a + 1; // cannot add `{integer}` to `Box<{integer}>`
}