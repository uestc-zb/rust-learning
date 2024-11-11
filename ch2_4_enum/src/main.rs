#[allow(unused_variables)]
fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);

    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);

    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

}


// #[allow(dead_code)]
// enum Option<T> {
//     Some(T),
//     None,
// }

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}


#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
  }

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",card);
}