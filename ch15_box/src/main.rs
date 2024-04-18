// use crate::List::{Cons,Nil};
// use std::cell::RefCell;
// use std::ops::Deref;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }
// fn main() {

//     let value = Rc::new(RefCell::new(5));
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);

//     // let b = Box::new(5);
//     // println!("b = {}", b);
//     // let list = Cons(1,
//     //     Box::new(Cons(2,
//     //         Box::new(Cons(3,
//     //             Box::new(Nil))))));
//     // let x = 5;
//     // let y = &x;
//     // let y = Box::new(x);
//     // let y = MyBox::new(x);
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);

//     let c = CustomSmartPointer{data: String::from("my stuff")};
//     let d = CustomSmartPointer{data: String::from("other stuff")};
//     // println!("CustomSamrtPinters created.");

//     let c = CustomSmartPointer { data: String::from("some data") };
//     // println!("created.00");
//     drop(c);
//     // println!("dropped");

//     // let a = Rc::new(Cons(5,
//     //     Rc::new(Cons(10,
//     //         Rc::new(Nil)))));
//     // println!("count after creating a = {}", Rc::strong_count(&a));
//     // let b = Cons(3, Rc::clone(&a));
//     // println!("count after creating b = {}", Rc::strong_count(&a));
//     // {
//     //     let c = Cons(4, Rc::clone(&a));
//     //     println!("count after creating c = {}", Rc::strong_count(&a));
//     // }
//     // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

// // enum List {
// //     Cons(i32, Rc<List>),
// //     Nil,
// // }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         // println!("Dropping CustomSmartPointer with data '{}'!", self.data);
//     }
// }



use std::{cell::Ref, rc::Rc};
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Mil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

   {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: u32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}