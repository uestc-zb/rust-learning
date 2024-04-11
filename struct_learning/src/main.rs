struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// struct Users {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
// struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someon@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{} {} {} {}",user1.active, user1.username,
     user1.email, user1.sign_in_count);

    let mut user1 = User {
        email: String::from("someon@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{} {} {} {}",user1.active, user1.username,
     user1.email, user1.sign_in_count);

    let user1 = build_user(String::from("email"), String::from("username"));
    println!("{} {} {} {}",user1.active, user1.username,
     user1.email, user1.sign_in_count);

    let user1 = build_user2(String::from("email"), String::from("username"));
    println!("{} {} {} {}",user1.active, user1.username,
     user1.email, user1.sign_in_count);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{} {} {} {}",user2.active, user2.username,
     user1.email, user1.sign_in_count);

    let user1 = User {
        email: String::from("someon@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@qq.com"),
        ..user1
    };
    println!("{} {} {} {}",user2.active, user2.username,
     user2.email, user2.sign_in_count);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {} {} {} {} {}", black.0, black.1, black.2,
     origin.0, origin.1, origin.2);

    // let object = AlwaysEqual;

    // let user1 = Users {
    //     email: "somecom",
    //     username: "somename",
    //     active: true,
    //     sign_in_count: 1,
    // };
    let width1 = 30;
    let height1 = 50;

    println!("The area is {}", area(width1, height1));

    let rect1 = (30, 50);
    println!("area is {}", area2(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area {}", area3(&rect1));
    println!("{:#?}", rect1);

    let rect2 = Rectangle {
        width: dbg!(30),
        height: 50,
    };
    dbg!(&rect2);

    let rect1 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("area {} {}", rect1.area(), rect1.width());

    println!("{}", rect1.can_hold(&rect2));
    
    println!("{}", Rectangle::square(3).area());
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}