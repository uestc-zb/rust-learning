fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(1);
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(1);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }

    // let v = vec![
    //     IpAddr::V4("127.0.0.1".to_string()),
    //     IpAddr::V6("::1".to_string())
    // ];

    // for ip in v {
    //     show_addr(ip)
    // }

    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
    let mut vec = vec![1, 5, 10, 2, 15];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}



// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String)
// }

trait IpAddr {
    fn display(&self);
}
struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

// fn show_addr(ip: IpAddr) {
//     println!("{:?}",ip);
// }

