// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
    
// }

// fn main() {
//     let x = Box::new(1);
//     let sum = *x + 1;
// }

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let y = MyBox::new(5);

//     assert_eq!(5, *y);
// }

fn main() {
    let s = String::from("hello world");
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}















