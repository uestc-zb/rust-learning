fn main() {
    // let v = vec![1, 2, 3];
    
    // v[99];

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();

}

// fn main() {
//     panic!("crash and burn");
// }


enum Result<T, E> {
    Ok(T),
    Err(E),
}
