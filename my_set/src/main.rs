#![allow(unused)]
use std::vec;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(5);
    // v.push(5);
    // v.push(5);
    // {
    //     let v = vec![1, 2, 3, 4, 5];
    // }

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // for i in &row {
    //     println!("{}", i);
    // }

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    
    
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{} {}",s2, s1);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from("hello");
    // let h = s1[0];
    let len = String::from("Hola").len();
    println!("{}", len);

    let len = String::from("Здравствуйте").len();
    println!("{}", len);
    
    let hello = "Здравствуйте";
    // let answer = &hello[0];
    let s = &hello[0..4];
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }
    
    for b in hello.bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    let field_name = String::from("Favorite color");
    let score = map.get(&field_name);
    // println!("{}",score);
    for (key, value) in &map {
        println!("{}:{}", key, value);
    }
    println!("{:?}", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}",scores);

    let text = "hello world woderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}


