#[allow(unused_variables)]
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("{}", user1.active);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
        
    };
    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);
}



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
// impl SomeTrait for AlwaysEqual {

// }


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
