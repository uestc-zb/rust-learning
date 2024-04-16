fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The {}", result);

    // let result = largest2(&number_list);

    // println!("The {}", result);

    let integer = Point {x:5, y:10};
    let float = Point {x:1.0, y:4.0};

    let p: Point<i32> = Point {x:5, y:10};
    println!("{}",p.x());
    let p: Point<f32> = Point {x:5.0, y:10.0};

    println!("{}",p.distance_from_origin());
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
// fn largest2<T> (list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }