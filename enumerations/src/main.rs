// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }


// fn main() {
//     // enum IpAddrKind {
//     //     V4,
//     //     V6,
//     // }

//     // struct IpAddr {
//     //     kind: IpAddrKind,
//     //     address: String,
//     // }
//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),


//     // enum IpAddr {
//     //     V4(String),
//     //     V6(String),
//     // }

//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     // let loopback = IpAddr::V6(String::from("::1"));

//     // enum IpAddr {
//     //     V4(u8, u8, u8, u8),
//     //     V6(String),
//     // }

//     // let home = IpAddr::V4(127,0,0,1);
//     // let loopback = IpAddr::V6(String::from("::1"));
//         value_in_cents(Coin::Quarter(UsState::Alabama));

// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky pennt!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarte form {:?}", state);
//             25
//         }
//     }

// }

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}