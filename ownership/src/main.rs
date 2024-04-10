fn main() {
    let s = "hello";
    println!("Hello, world! {}", s);
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world! {}", s1, s2);

    let s = String::from("hello");
    takes_own(s);
    let x = 5;
    make_copy(x);

    let s1 = gives_own();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);
    let s1 = String::from("hello");
    let len = cal_length(&s1);
    println!("{} {}",s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}",r1);
    }
    let r2 = &mut s;
    println!("{}",r2);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{}", word);
    println!("{}", s);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hel lo");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let word = first_word2(&s);
    println!("{}", word);
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in  bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn cal_length(s: &String) -> usize{
    s.len()
}

fn gives_own() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


fn takes_own(some_string: String){
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}