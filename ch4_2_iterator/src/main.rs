// fn main() {
//     println!("Hello, world!");
//     let arr = [1, 2, 3];
//     for v in arr {
//         println!("{}",v);
//     }

//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("{}", val);
//     }

// }

// fn main() {
//     let arr = [1, 2, 3];
//     let mut arr_iter = arr.into_iter();

//     assert_eq!(arr_iter.next(), Some(1));
//     assert_eq!(arr_iter.next(), Some(2));
//     assert_eq!(arr_iter.next(), Some(3));
//     assert_eq!(arr_iter.next(), None);

//     let values = vec![1, 2, 3];

// {
//     let result = match IntoIterator::into_iter(values) {
//         mut iter => loop {
//             match iter.next() {
//                 Some(x) => { println!("{}", x); },
//                 None => break,
//             }
//         },
//     };
//     result
// }

// }

// fn main() {
//     let values = vec![1, 2, 3, 4];

//     for v in values.into_iter().into_iter().into_iter() {
//         println!("{}",v)
//     }
// }

// fn main() {
//     let values = vec![1, 2, 3];

//     for v in values.into_iter() {
//         println!("{}", v)
//     }

//     // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
//     // println!("{:?}",values);

//     let values = vec![1, 2, 3];
//     let _values_iter = values.iter();

//     // // 不会报错，因为 values_iter 只是借用了 values 中的元素
//     println!("{:?}", values);

//     let mut values = vec![1, 2, 3];
//     // 对 values 中的元素进行可变借用
//     let mut values_iter_mut = values.iter_mut();

//     // // 取出第一个元素，并修改为0
//     if let Some(v) = values_iter_mut.next() {
//         *v = 0;
//     }

//     // // 输出[0, 2, 3]
//     println!("{:?}", values);
// }


// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);

//     // v1_iter 是借用了 v1，因此 v1 可以照常使用
//     println!("{:?}",v1);

//     // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
//     // println!("{:?}",v1_iter);

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     assert_eq!(v2, vec![2, 3, 4]);

// }

use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}",folks);

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    
    let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
    assert_eq!(18, sum);
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i,v) in v.iter().enumerate() {
        println!("第{}个值是{}",i,v)
    }

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);

    println!("{}", val);



}


struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
